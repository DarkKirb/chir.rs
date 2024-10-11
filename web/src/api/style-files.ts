import { decode as cborDecode } from "cbor-x";
import * as v from "valibot";

export interface Stylesheets {
  baseStyle: string;
  sunsetTheme: string;
  transRightsTheme: string;
  blackTheme: string;
  whiteTheme: string;
}

const jsonSchema = v.object({
  baseStyle: v.pipe(v.string(), v.url()),
  sunsetTheme: v.pipe(v.string(), v.url()),
  transRightsTheme: v.pipe(v.string(), v.url()),
  blackTheme: v.pipe(v.string(), v.url()),
  whiteTheme: v.pipe(v.string(), v.url()),
});

const cborSchema = v.object({
  0: v.pipe(v.string(), v.url()),
  1: v.pipe(v.string(), v.url()),
  2: v.pipe(v.string(), v.url()),
  3: v.pipe(v.string(), v.url()),
  4: v.pipe(v.string(), v.url()),
});

export function fromJson(json: string): Stylesheets {
  let data = JSON.parse(json);
  return v.parse(jsonSchema, data);
}

export function fromCBOR(cbor: ArrayBuffer): Stylesheets {
  let data = cborDecode(new Uint8Array(cbor));
  let dataParsed = v.parse(cborSchema, data);
  return {
    baseStyle: dataParsed[0],
    sunsetTheme: dataParsed[1],
    transRightsTheme: dataParsed[2],
    blackTheme: dataParsed[3],
    whiteTheme: dataParsed[4],
  };
}

export async function fetchStylesheets(): Promise<Stylesheets> {
  const response = await fetch("/api/style-files", {
    headers: {
      Accept: "application/cbor, application/json;q=0.9",
    },
  });

  if (!response.ok) {
    throw new Error(
      `Failed to load stylesheets from server: ${response.statusText}`,
    );
  }

  switch (response.headers.get("Content-Type")) {
    case "application/json":
      return fromJson(await response.text());
    case "application/cbor":
      return fromCBOR(await response.arrayBuffer());
    default:
      throw new Error(
        `Invalid content type ${response.headers.get("Content-Type")}`,
      );
  }
}
