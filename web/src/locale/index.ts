import { FluentBundle, FluentResource, FluentVariable } from "@fluent/bundle";
import Cookies from "js-cookie";
import "tslib";
import { LanguageId } from "./LanguageId";

const LANGUAGE_IDS: Record<string, string> = {
  de: "de-DE",
  en: "en-US",
  "en-IE": "en-GB",
  "en-Sten-US": "en-Sten-US",
  "en-GB": "en-GB",
  fr: "fr-FR",
  jbo: "jbo-Latn-XX",
  "jbo-Zblr-XX": "jbo-Zblr-XX",
  nl: "nl-NL",
  tok: "tok-Latn-XX",
  "tok-Emsi-XX": "tok-Emsi-XX",
  "tok-Hani-XX": "tok-Hani-XX",
  "tok-Stln-XX": "tok-Stln-XX",
};

function lookupLanguage(langId: string): string | null {
  let lang: LanguageId;
  try {
    lang = new LanguageId(langId);
  } catch (e) {
    return null;
  }
  do {
    const id = LANGUAGE_IDS[lang.toString()];
    if (id) {
      return id;
    }
  } while (lang.removeLeastSignificantComponent());
  return null;
}

const loadedLanguages: Record<string, FluentBundle> = {};

async function pickBundle(languageId: string): Promise<string> {
  switch (languageId) {
    case "de":
      return (await import("../../../locales/de-DE/main.ftl")).default;
    case "de-DE":
      return (await import("../../../locales/de-DE/main.ftl")).default;
    case "en":
      return (await import("../../../locales/en-US/main.ftl")).default;
    case "en-US":
      return (await import("../../../locales/en-US/main.ftl")).default;
    case "en-IE":
      return (await import("../../../locales/en-GB/main.ftl")).default;
    case "en-Sten-US":
      return (await import("../../../locales/en-Sten-US/main.ftl")).default;
    case "en-Sten-US-display":
      return (await import("../../../locales/en-Sten-US/main.ftl")).default;
    case "en-Sten-US-runes":
      return (await import("../../../locales/en-Sten-US/main.ftl")).default;
    case "en-GB":
      return (await import("../../../locales/en-GB/main.ftl")).default;
    case "fr":
      return (await import("../../../locales/fr-FR/main.ftl")).default;
    case "fr-FR":
      return (await import("../../../locales/fr-FR/main.ftl")).default;
    case "jbo":
      return (await import("../../../locales/jbo-Latn-XX/main.ftl")).default;
    case "jbo-XX":
      return (await import("../../../locales/jbo-Latn-XX/main.ftl")).default;
    case "jbo-Latn-XX":
      return (await import("../../../locales/jbo-Latn-XX/main.ftl")).default;
    case "jbo-Zblr-XX":
      return (await import("../../../locales/jbo-Zblr-XX/main.ftl")).default;
    case "nl":
      return (await import("../../../locales/nl-NL/main.ftl")).default;
    case "nl-NL":
      return (await import("../../../locales/nl-NL/main.ftl")).default;
    case "tok":
      return (await import("../../../locales/tok-Latn-XX/main.ftl")).default;
    case "tok-XX":
      return (await import("../../../locales/tok-Latn-XX/main.ftl")).default;
    case "tok-Emsi-XX":
      return (await import("../../../locales/tok-Emsi-XX/main.ftl")).default;
    case "tok-Hani-XX":
      return (await import("../../../locales/tok-Hani-XX/main.ftl")).default;
    case "tok-Latn-XX":
      return (await import("../../../locales/tok-Latn-XX/main.ftl")).default;
    case "tok-Stln-XX":
      return (await import("../../../locales/tok-Stln-XX/main.ftl")).default;
  }
  throw new Error("Unknown language: " + languageId);
}

async function loadLanguage(lang: string): Promise<FluentBundle> {
  if (lang in loadedLanguages) {
    return loadedLanguages[lang];
  }

  const fluentFile = await pickBundle(lang);
  const fluentResource = new FluentResource(fluentFile);
  loadedLanguages[lang] = new FluentBundle(lang);
  loadedLanguages[lang].addResource(fluentResource);
  return loadedLanguages[lang];
}

function setRootLanguage(lang: string) {
  document.documentElement.lang = lang;
}

function validateFluentArgs(
  json: string,
): Record<string, FluentVariable> | null {
  const args: unknown = JSON.parse(json);
  if (args === null || args === undefined) {
    return null;
  }
  if (typeof args !== "object") {
    throw new Error("Fluent arguments must be an object.");
  }
  const result: Record<string, FluentVariable> = {};
  for (const key in args) {
    if (Object.prototype.hasOwnProperty.call(args, key)) {
      const value: unknown = args[key];
      if (typeof value === "number") {
        result[key] = value;
      } else if (typeof value === "string") {
        // It could be a date
        const date = new Date(value);
        if (!isNaN(date.getTime())) {
          result[key] = date;
        } else {
          result[key] = value;
        }
      }
      // TODO: support FluentValue
    }
  }
  return result;
}

async function localize(langs: string[]) {
  const bundles: FluentBundle[] = [];
  for (const lang of langs) {
    const parsedLang = lookupLanguage(lang);
    if (parsedLang === null) continue;
    bundles.push(await loadLanguage(parsedLang));
  }

  document.querySelectorAll("[data-l10n-id]").forEach((element) => {
    const l10nId = element.getAttribute("data-l10n-id") ?? "undefined";
    for (const bundle of bundles) {
      const message = bundle.getMessage(l10nId);
      if (message?.value) {
        let fluentVariable = validateFluentArgs(
          element.getAttribute("data-l10n-args") ?? "{}",
        );
        if (fluentVariable === null) {
          fluentVariable = {};
        }
        const content = bundle.formatPattern(message.value, fluentVariable);
        const propertyAttribute = element.getAttribute("data-l10n-property");
        if (propertyAttribute !== null) {
          element.setAttribute(propertyAttribute, content);
        } else {
          element.textContent = content;
        }
        break;
      }
    }
  });
  setRootLanguage(langs[0]);
}

function* findBrowserLanguageIds(): Generator<string> {
  for (const lang of navigator.languages) {
    if (LANGUAGE_IDS[lang]) {
      yield LANGUAGE_IDS[lang];
    }
  }
}

async function* getUserLanguageIds(): AsyncGenerator<string> {
  const Cookies = (await import("js-cookie")).default;
  const lang = Cookies.get("locale");
  if (lang) {
    yield lang;
  }

  yield* findBrowserLanguageIds();

  yield "en-US";
}

async function localizeAll(): Promise<void> {
  const langs = [];
  for await (const lang of getUserLanguageIds()) {
    langs.push(lang);
  }
  await localize(langs);
}

export async function handleLanguageChange(event: Event) {
  const target = event.target as HTMLSelectElement;
  if (target.value === "auto") {
    Cookies.remove("locale");
  } else {
    Cookies.set("locale", target.value, { expires: 365 });
  }
  await localizeAll();
}
