#!/usr/bin/env node
import { TinyHumansClient } from "./client.js";

interface CliOptions {
  baseUrl: string;
  token?: string;
  apiKey?: string;
  adminServiceToken?: string;
  body?: string;
  raw?: boolean;
}

const USAGE = `Usage:
  tinyhumans health [--base-url URL]
  tinyhumans swagger [--base-url URL]
  tinyhumans get PATH [--token TOKEN] [--api-key KEY]
  tinyhumans post PATH [--body JSON] [--token TOKEN] [--api-key KEY]
  tinyhumans put PATH [--body JSON] [--token TOKEN] [--api-key KEY]
  tinyhumans patch PATH [--body JSON] [--token TOKEN] [--api-key KEY]
  tinyhumans delete PATH [--token TOKEN] [--api-key KEY]

Environment:
  TINYHUMANS_BASE_URL, TINYHUMANS_TOKEN, TINYHUMANS_API_KEY, TINYHUMANS_ADMIN_SERVICE_TOKEN
`;

async function main(): Promise<void> {
  const [command, path, ...rest] = process.argv.slice(2);
  if (!command || command === "--help" || command === "-h") {
    console.log(USAGE);
    return;
  }

  const options = parseOptions(rest);
  const client = new TinyHumansClient({
    baseUrl: options.baseUrl,
    token: options.token,
    apiKey: options.apiKey,
    adminServiceToken: options.adminServiceToken,
    unwrapEnvelope: !options.raw,
  });

  let result: unknown;
  if (command === "health") {
    result = await client.health.check();
  } else if (command === "swagger") {
    result = await client.swagger();
  } else if (["get", "post", "put", "patch", "delete"].includes(command)) {
    if (!path) throw new Error(`${command} requires PATH`);
    const body = ["post", "put", "patch"].includes(command)
      ? parseBody(options.body ?? (await readStdinIfAvailable()))
      : undefined;
    result = await client.raw.request(command.toUpperCase(), path, {
      body,
      unwrapEnvelope: !options.raw,
    });
  } else {
    throw new Error(`Unknown command: ${command}\n\n${USAGE}`);
  }

  print(result);
}

function parseOptions(args: string[]): CliOptions {
  const options: CliOptions = {
    baseUrl: process.env.TINYHUMANS_BASE_URL ?? "https://api.tinyhumans.ai",
    token: process.env.TINYHUMANS_TOKEN,
    apiKey: process.env.TINYHUMANS_API_KEY,
    adminServiceToken: process.env.TINYHUMANS_ADMIN_SERVICE_TOKEN,
  };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    const next = args[index + 1];
    if (arg === "--base-url" && next) {
      options.baseUrl = next;
      index += 1;
    } else if (arg === "--token" && next) {
      options.token = next;
      index += 1;
    } else if (arg === "--api-key" && next) {
      options.apiKey = next;
      index += 1;
    } else if (arg === "--admin-service-token" && next) {
      options.adminServiceToken = next;
      index += 1;
    } else if (arg === "--body" && next) {
      options.body = next;
      index += 1;
    } else if (arg === "--raw") {
      options.raw = true;
    } else {
      throw new Error(`Unknown option: ${arg}`);
    }
  }
  return options;
}

function parseBody(body: string | undefined): unknown {
  if (!body || !body.trim()) return undefined;
  return JSON.parse(body);
}

async function readStdinIfAvailable(): Promise<string | undefined> {
  if (process.stdin.isTTY) return undefined;
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) chunks.push(Buffer.from(chunk));
  return Buffer.concat(chunks).toString("utf8");
}

function print(value: unknown): void {
  if (typeof value === "string") {
    console.log(value);
    return;
  }
  console.log(JSON.stringify(value, null, 2));
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
