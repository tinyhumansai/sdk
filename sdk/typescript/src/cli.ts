#!/usr/bin/env node
import { TinyHumansClient } from "./client.js";
import { COMMANDS, type CliCommand } from "./cli-commands.js";

interface GlobalOptions {
  baseUrl: string;
  token?: string;
  apiKey?: string;
  adminServiceToken?: string;
  raw: boolean;
  pretty: boolean;
}

const RAW_VERBS = ["get", "post", "put", "patch", "delete"] as const;

function topUsage(): string {
  const namespaces = [...new Set(COMMANDS.map((c) => c.namespace))].sort();
  return `tinyhumans — typed CLI for the TinyHumans backend

Usage:
  tinyhumans <namespace> <command> [args...] [flags]
  tinyhumans <namespace>                 list commands in a namespace
  tinyhumans <namespace> <command> --help  show a command's signature
  tinyhumans list [--json]               list every command
  tinyhumans swagger                     fetch the OpenAPI document
  tinyhumans raw <${RAW_VERBS.join("|")}> PATH [--body JSON]  raw request

Namespaces (${namespaces.length}):
  ${namespaces.join(", ")}

Global flags:
  --base-url URL            (env TINYHUMANS_BASE_URL, default https://api.tinyhumans.ai)
  --token TOKEN             (env TINYHUMANS_TOKEN)
  --api-key KEY             (env TINYHUMANS_API_KEY)
  --admin-service-token TOK (env TINYHUMANS_ADMIN_SERVICE_TOKEN)
  --raw                     do not unwrap the { success, data } envelope
  --json                    compact JSON output (default is pretty)

Argument model per command:
  positional params -> passed in order (or via --<name> VALUE)
  request body      -> --body JSON  (or piped via stdin)
  query params      -> --query key=value (repeatable)`;
}

function commandSignature(cmd: CliCommand): string {
  const parts = cmd.params.map((p) =>
    p.role === "positional" ? `<${p.name}>` : p.role === "body" ? "--body JSON" : "--query k=v",
  );
  const flags = cmd.nonEnvelope ? "  (returns raw provider response)" : "";
  return `tinyhumans ${cmd.namespace} ${cmd.command} ${parts.join(" ")}\n  ${cmd.verb} ${cmd.path}${flags}`;
}

function listNamespace(namespace: string): string {
  const cmds = COMMANDS.filter((c) => c.namespace === namespace);
  if (cmds.length === 0) return `Unknown namespace: ${namespace}`;
  const lines = cmds.map((c) => {
    const args = c.params
      .map((p) => (p.role === "positional" ? `<${p.name}>` : p.role === "body" ? "--body" : "--query"))
      .join(" ");
    return `  ${c.command.padEnd(34)} ${c.verb.padEnd(6)} ${c.path}${args ? `   [${args}]` : ""}`;
  });
  return `${namespace} commands (${cmds.length}):\n${lines.join("\n")}`;
}

async function main(): Promise<void> {
  const argv = process.argv.slice(2);
  const { positionals, flags } = splitArgs(argv);
  const first = positionals[0];

  if (!first || first === "help" || (flags.help !== undefined && positionals.length === 0)) {
    console.log(topUsage());
    return;
  }

  if (first === "list") {
    if (flags.json !== undefined) {
      print(COMMANDS, true);
    } else {
      const grouped = [...new Set(COMMANDS.map((c) => c.namespace))]
        .sort()
        .map((ns) => listNamespace(ns))
        .join("\n\n");
      console.log(grouped);
    }
    return;
  }

  const options = resolveGlobals(flags);
  const client = new TinyHumansClient({
    baseUrl: options.baseUrl,
    token: options.token,
    apiKey: options.apiKey,
    adminServiceToken: options.adminServiceToken,
    unwrapEnvelope: !options.raw,
  });

  if (first === "swagger") {
    print(await client.swagger(), !options.pretty);
    return;
  }

  if (first === "raw") {
    const verb = positionals[1]?.toLowerCase();
    const path = positionals[2];
    if (!verb || !RAW_VERBS.includes(verb as (typeof RAW_VERBS)[number]) || !path) {
      throw new Error(`Usage: tinyhumans raw <${RAW_VERBS.join("|")}> PATH [--body JSON]`);
    }
    const body = ["post", "put", "patch"].includes(verb)
      ? parseJson(asStr(flags.body) ?? (await readStdin()))
      : undefined;
    const result = await client.raw.request(verb.toUpperCase(), path, {
      body,
      unwrapEnvelope: !options.raw,
    });
    print(result, !options.pretty);
    return;
  }

  // Namespace / command dispatch.
  const namespace = first;
  const namespaceCmds = COMMANDS.filter((c) => c.namespace === namespace);
  if (namespaceCmds.length === 0) {
    throw new Error(`Unknown namespace: ${namespace}\n\n${topUsage()}`);
  }
  const commandName = positionals[1];
  if (!commandName) {
    console.log(listNamespace(namespace));
    return;
  }
  const cmd = namespaceCmds.find((c) => c.command === commandName);
  if (!cmd) {
    throw new Error(`Unknown command: ${namespace} ${commandName}\n\n${listNamespace(namespace)}`);
  }
  if (flags.help !== undefined) {
    console.log(commandSignature(cmd));
    return;
  }

  const result = await invoke(client, cmd, positionals.slice(2), flags);
  print(result, !options.pretty);
}

async function invoke(
  client: TinyHumansClient,
  cmd: CliCommand,
  positionalArgs: string[],
  flags: Flags,
): Promise<unknown> {
  const queue = [...positionalArgs];
  const args: unknown[] = [];

  for (const param of cmd.params) {
    if (param.role === "positional") {
      const value = asStr(flags[param.name]) ?? queue.shift();
      if (value === undefined) {
        throw new Error(`Missing required argument <${param.name}> for ${cmd.namespace} ${cmd.command}`);
      }
      args.push(value);
    } else if (param.role === "body") {
      const body = parseJson(asStr(flags.body) ?? (await readStdin()));
      args.push(body);
    } else {
      args.push(buildQuery(flags));
    }
  }

  // Trim trailing undefined (optional body/query the user did not supply).
  while (args.length && args[args.length - 1] === undefined) args.pop();

  const nsClient = (client as unknown as Record<string, Record<string, (...a: unknown[]) => Promise<unknown>>>)[
    cmd.namespace
  ];
  const method = nsClient?.[cmd.method];
  if (!nsClient || typeof method !== "function") {
    throw new Error(`Method ${cmd.namespace}.${cmd.method} is not available`);
  }
  return method.apply(nsClient, args);
}

function buildQuery(flags: Flags): Record<string, unknown> | undefined {
  const entries = flags.query;
  if (!entries) return undefined;
  const list = Array.isArray(entries) ? entries : [entries];
  const query: Record<string, unknown> = {};
  for (const pair of list) {
    const eq = pair.indexOf("=");
    if (eq === -1) continue;
    const key = pair.slice(0, eq);
    const raw = pair.slice(eq + 1);
    query[key] = tryJson(raw);
  }
  return Object.keys(query).length ? query : undefined;
}

type Flags = Record<string, string | string[] | undefined> & { help?: string; json?: string };

function splitArgs(argv: string[]): { positionals: string[]; flags: Flags } {
  const positionals: string[] = [];
  const flags: Flags = {};
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === undefined) continue;
    if (arg.startsWith("--")) {
      const key = arg.slice(2);
      const boolean = key === "raw" || key === "json" || key === "help";
      const value = boolean ? "" : argv[++i];
      if (key === "query") {
        const existing = flags.query;
        flags.query = existing
          ? [...(Array.isArray(existing) ? existing : [existing]), value ?? ""]
          : [value ?? ""];
      } else {
        flags[key] = value ?? "";
      }
    } else {
      positionals.push(arg);
    }
  }
  return { positionals, flags };
}

function resolveGlobals(flags: Flags): GlobalOptions {
  const str = (v: string | string[] | undefined): string | undefined =>
    Array.isArray(v) ? v[0] : v;
  return {
    baseUrl: str(flags["base-url"]) ?? process.env.TINYHUMANS_BASE_URL ?? "https://api.tinyhumans.ai",
    token: str(flags.token) ?? process.env.TINYHUMANS_TOKEN,
    apiKey: str(flags["api-key"]) ?? process.env.TINYHUMANS_API_KEY,
    adminServiceToken:
      str(flags["admin-service-token"]) ?? process.env.TINYHUMANS_ADMIN_SERVICE_TOKEN,
    raw: flags.raw !== undefined,
    pretty: flags.json === undefined,
  };
}

function asStr(value: string | string[] | undefined): string | undefined {
  return Array.isArray(value) ? value[0] : value;
}

function parseJson(body: string | undefined): unknown {
  if (!body || !body.trim()) return undefined;
  return JSON.parse(body);
}

function tryJson(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch {
    return value;
  }
}

async function readStdin(): Promise<string | undefined> {
  if (process.stdin.isTTY) return undefined;
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) chunks.push(Buffer.from(chunk));
  const text = Buffer.concat(chunks).toString("utf8");
  return text.trim() ? text : undefined;
}

function print(value: unknown, compact = false): void {
  if (typeof value === "string") {
    console.log(value);
    return;
  }
  console.log(JSON.stringify(value, null, compact ? 0 : 2));
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
