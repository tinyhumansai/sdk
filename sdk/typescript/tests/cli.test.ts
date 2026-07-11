import { describe, expect, it } from "vitest";

import {
  asStr,
  buildQuery,
  commandSignature,
  defaultDeps,
  format,
  isMainEntry,
  listNamespace,
  parseJson,
  resolveGlobals,
  run,
  splitArgs,
  tryJson,
  type RunDeps,
} from "../src/cli.js";
import { COMMANDS } from "../src/cli-commands.js";

describe("cli pure helpers", () => {
  it("splitArgs parses flag values, boolean flags, and repeated --query", () => {
    const { positionals, flags } = splitArgs([
      "auth",
      "me",
      "--token",
      "tok",
      "--raw",
      "--json",
      "--help",
      "--query",
      "a=1",
      "--query",
      "b=2",
    ]);
    expect(positionals).toEqual(["auth", "me"]);
    expect(flags.token).toBe("tok");
    expect(flags.raw).toBe("");
    expect(flags.json).toBe("");
    expect(flags.help).toBe("");
    expect(flags.query).toEqual(["a=1", "b=2"]);
  });

  it("splitArgs treats a single --query as an array and skips a missing trailing value", () => {
    const { flags } = splitArgs(["--query", "a=1"]);
    expect(flags.query).toEqual(["a=1"]);
    const { flags: flags2 } = splitArgs(["--token"]);
    expect(flags2.token).toBe("");
  });

  it("resolveGlobals prefers flags then env then defaults", () => {
    const env = {
      TINYHUMANS_BASE_URL: "https://env.example",
      TINYHUMANS_TOKEN: "env-tok",
      TINYHUMANS_API_KEY: "env-key",
      TINYHUMANS_ADMIN_SERVICE_TOKEN: "env-admin",
    };
    const fromFlags = resolveGlobals(
      { "base-url": "https://flag.example", token: "flag-tok", raw: "", json: "" },
      env,
    );
    expect(fromFlags.baseUrl).toBe("https://flag.example");
    expect(fromFlags.token).toBe("flag-tok");
    expect(fromFlags.apiKey).toBe("env-key");
    expect(fromFlags.adminServiceToken).toBe("env-admin");
    expect(fromFlags.raw).toBe(true);
    expect(fromFlags.pretty).toBe(false);

    const defaults = resolveGlobals({}, {});
    expect(defaults.baseUrl).toBe("https://api.tinyhumans.ai");
    expect(defaults.token).toBeUndefined();
    expect(defaults.raw).toBe(false);
    expect(defaults.pretty).toBe(true);
  });

  it("buildQuery parses key=value pairs, JSON values, and skips malformed entries", () => {
    expect(buildQuery({})).toBeUndefined();
    expect(buildQuery({ query: [] })).toBeUndefined();
    expect(buildQuery({ query: "noequals" })).toBeUndefined();
    expect(buildQuery({ query: ["a=1", "b=hello", "c={\"x\":1}"] })).toEqual({
      a: 1,
      b: "hello",
      c: { x: 1 },
    });
    expect(buildQuery({ query: "single=2" })).toEqual({ single: 2 });
  });

  it("format renders strings verbatim and objects as pretty or compact JSON", () => {
    expect(format("hi")).toBe("hi");
    expect(format({ a: 1 })).toBe("{\n  \"a\": 1\n}");
    expect(format({ a: 1 }, true)).toBe('{"a":1}');
  });

  it("tryJson parses valid JSON and returns the raw string otherwise", () => {
    expect(tryJson("42")).toBe(42);
    expect(tryJson("not json")).toBe("not json");
  });

  it("parseJson returns undefined for empty input and parses otherwise", () => {
    expect(parseJson(undefined)).toBeUndefined();
    expect(parseJson("   ")).toBeUndefined();
    expect(parseJson('{"a":1}')).toEqual({ a: 1 });
  });

  it("asStr flattens arrays to the first element", () => {
    expect(asStr("x")).toBe("x");
    expect(asStr(["a", "b"])).toBe("a");
    expect(asStr(undefined)).toBeUndefined();
  });

  it("commandSignature renders positional, body, and query params", () => {
    const cmd = COMMANDS.find((c) => c.namespace === "teams" && c.command === "update-team")!;
    const sig = commandSignature(cmd);
    expect(sig).toContain("tinyhumans teams update-team <teamId> --body JSON");
    expect(sig).toContain("PUT /teams/:teamId");

    const nonEnvelope = COMMANDS.find((c) => c.nonEnvelope)!;
    expect(commandSignature(nonEnvelope)).toContain("(returns raw provider response)");
  });

  it("listNamespace lists commands and reports unknown namespaces", () => {
    const listed = listNamespace("auth");
    expect(listed).toContain("auth commands");
    expect(listed).toContain("me");
    expect(listNamespace("nope")).toBe("Unknown namespace: nope");
  });

  it("isMainEntry is false for undefined argv", () => {
    expect(isMainEntry(undefined)).toBe(false);
    expect(isMainEntry("/some/other/file.js")).toBe(false);
  });

  it("defaultDeps exposes the expected shape", () => {
    expect(typeof defaultDeps.createClient).toBe("function");
    expect(typeof defaultDeps.readStdin).toBe("function");
    expect(typeof defaultDeps.write).toBe("function");
    expect(defaultDeps.env).toBe(process.env);
  });
});

interface StubClient {
  auth: {
    me: (...a: unknown[]) => Promise<unknown>;
    verifyEmail: (...a: unknown[]) => Promise<unknown>;
  };
  teams: { updateTeam: (...a: unknown[]) => Promise<unknown> };
  inference: { listModels: (...a: unknown[]) => Promise<unknown> };
  raw: { request: (...a: unknown[]) => Promise<unknown> };
  swagger: () => Promise<unknown>;
}

function makeDeps(overrides: Partial<RunDeps> = {}) {
  const lines: string[] = [];
  const createCalls: unknown[] = [];
  const rawCalls: unknown[][] = [];
  const stub: StubClient = {
    auth: {
      me: async () => ({ id: "u" }),
      verifyEmail: async (token: unknown) => ({ verified: token }),
    },
    teams: {
      updateTeam: async (id: unknown, body: unknown) => ({ id, body }),
    },
    inference: {
      listModels: async (query: unknown) => ({ query }),
    },
    raw: {
      request: async (...args: unknown[]) => {
        rawCalls.push(args);
        return { ok: true };
      },
    },
    swagger: async () => ({ openapi: "3" }),
  };
  const deps: RunDeps = {
    createClient: (options) => {
      createCalls.push(options);
      return stub as never;
    },
    readStdin: async () => '{"stdin":true}',
    write: (line) => lines.push(line),
    env: {},
    ...overrides,
  };
  return { deps, lines, createCalls, rawCalls, stub };
}

describe("cli run dispatch", () => {
  it("prints top usage when given no args", async () => {
    const { deps, lines } = makeDeps();
    await run([], deps);
    expect(lines[0]).toContain("tinyhumans — typed CLI");
  });

  it("prints top usage for `help`", async () => {
    const { deps, lines } = makeDeps();
    await run(["help"], deps);
    expect(lines[0]).toContain("Usage:");
  });

  it("prints top usage for a lone --help flag", async () => {
    const { deps, lines } = makeDeps();
    await run(["--help"], deps);
    expect(lines[0]).toContain("tinyhumans — typed CLI");
  });

  it("lists every command grouped by namespace", async () => {
    const { deps, lines } = makeDeps();
    await run(["list"], deps);
    expect(lines[0]).toContain("auth commands");
    expect(lines[0]).toContain("teams commands");
  });

  it("lists commands as JSON with --json", async () => {
    const { deps, lines } = makeDeps();
    await run(["list", "--json"], deps);
    const parsed = JSON.parse(lines[0]!);
    expect(Array.isArray(parsed)).toBe(true);
    expect(parsed.length).toBe(COMMANDS.length);
  });

  it("lists a namespace when no command is given", async () => {
    const { deps, lines } = makeDeps();
    await run(["auth"], deps);
    expect(lines[0]).toContain("auth commands");
  });

  it("prints a command signature with --help", async () => {
    const { deps, lines } = makeDeps();
    await run(["teams", "update-team", "--help"], deps);
    expect(lines[0]).toContain("tinyhumans teams update-team <teamId>");
  });

  it("dispatches a no-arg positional command", async () => {
    const { deps, lines } = makeDeps();
    await run(["auth", "me"], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ id: "u" });
  });

  it("dispatches a positional argument", async () => {
    const { deps, lines } = makeDeps();
    await run(["auth", "verify-email", "tok_123"], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ verified: "tok_123" });
  });

  it("dispatches a path param plus --body", async () => {
    const { deps, lines } = makeDeps();
    await run(["teams", "update-team", "team_1", "--body", '{"name":"new"}'], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ id: "team_1", body: { name: "new" } });
  });

  it("reads the body from stdin when --body is absent", async () => {
    const { deps, lines } = makeDeps();
    await run(["teams", "update-team", "team_2"], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ id: "team_2", body: { stdin: true } });
  });

  it("dispatches a --query command", async () => {
    const { deps, lines } = makeDeps();
    await run(["inference", "list-models", "--query", "limit=5"], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ query: { limit: 5 } });
  });

  it("fetches the swagger document", async () => {
    const { deps, lines } = makeDeps();
    await run(["swagger"], deps);
    expect(JSON.parse(lines[0]!)).toEqual({ openapi: "3" });
  });

  it("runs a raw GET request", async () => {
    const { deps, lines, rawCalls } = makeDeps();
    await run(["raw", "get", "/x"], deps);
    expect(rawCalls[0]![0]).toBe("GET");
    expect(rawCalls[0]![1]).toBe("/x");
    expect((rawCalls[0]![2] as { body?: unknown }).body).toBeUndefined();
    expect(JSON.parse(lines[0]!)).toEqual({ ok: true });
  });

  it("runs a raw POST request reading the body from stdin", async () => {
    const { deps, rawCalls } = makeDeps();
    await run(["raw", "post", "/x"], deps);
    expect(rawCalls[0]![0]).toBe("POST");
    expect((rawCalls[0]![2] as { body?: unknown }).body).toEqual({ stdin: true });
  });

  it("runs a raw POST request with an explicit --body", async () => {
    const { deps, rawCalls } = makeDeps();
    await run(["raw", "post", "/x", "--body", '{"a":1}'], deps);
    expect((rawCalls[0]![2] as { body?: unknown }).body).toEqual({ a: 1 });
  });

  it("throws on an invalid raw verb", async () => {
    const { deps } = makeDeps();
    await expect(run(["raw", "fly", "/x"], deps)).rejects.toThrow(/Usage: tinyhumans raw/);
  });

  it("passes global options through to createClient (raw disables unwrapping)", async () => {
    const { deps, createCalls } = makeDeps({ env: { TINYHUMANS_TOKEN: "env-tok" } });
    await run(["auth", "me", "--raw", "--api-key", "k"], deps);
    expect(createCalls[0]).toMatchObject({
      token: "env-tok",
      apiKey: "k",
      unwrapEnvelope: false,
    });
  });

  it("throws on an unknown namespace", async () => {
    const { deps } = makeDeps();
    await expect(run(["nope"], deps)).rejects.toThrow(/Unknown namespace: nope/);
  });

  it("throws on an unknown command", async () => {
    const { deps } = makeDeps();
    await expect(run(["auth", "nope"], deps)).rejects.toThrow(/Unknown command: auth nope/);
  });

  it("throws when a required argument is missing", async () => {
    const { deps } = makeDeps();
    await expect(run(["auth", "verify-email"], deps)).rejects.toThrow(/Missing required argument/);
  });

  it("throws when the target method is unavailable on the client", async () => {
    const { deps } = makeDeps({
      createClient: () => ({ auth: {} }) as never,
    });
    await expect(run(["auth", "me"], deps)).rejects.toThrow(/is not available/);
  });
});
