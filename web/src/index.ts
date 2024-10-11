async function getLanguageCookie(): Promise<string | undefined> {
  const Cookies = await import("js-cookie");
  return Cookies.default.get("_LANG");
}

async function getThemeCookie(): Promise<string | undefined> {
  const Cookies = await import("js-cookie");
  return Cookies.default.get("_THEME");
}

async function getTheme(): Promise<string> {
  const themeCookie = await getThemeCookie();
  if (themeCookie) {
    return themeCookie;
  } else {
    return "auto";
  }
}

let styleSheets: any = undefined;

function appendStylesheet(url: string, media?: string): void {
  let link = document.createElement("link");
  link.setAttribute("rel", "stylesheet");
  link.setAttribute("href", url);
  if (media !== undefined) {
    link.setAttribute("media", media);
  }
  document.head.appendChild(link);
}

async function applyTheme(theme: string | undefined): Promise<void> {
  if (styleSheets === undefined) {
    const response = await fetch("/api/style-files", {
      headers: {
        Accept: "application/cbor",
      },
    });

    const styleSheetsCbor = await response.arrayBuffer();
    const cbor = await import("cbor-x");
    const styleSheetsData = cbor.decode(new Uint8Array(styleSheetsCbor));
    styleSheets = styleSheetsData;
  }

  if (styleSheets !== undefined) {
    document.querySelectorAll("link[rel=stylesheet]").forEach;
    Array.from(document.getElementsByTagName("link"))
      .filter((e) => e.getAttribute("rel") === "stylesheet")
      .forEach((e) => {
        e.remove();
      });

    // Now add the new styles back
    let head = document.head;
    appendStylesheet(styleSheets[0]);
    switch (theme) {
      case "sunset":
        appendStylesheet(styleSheets[1]);
        break;
      case "trans-rights":
        appendStylesheet(styleSheets[2]);
        break;
      case "black":
        appendStylesheet(styleSheets[3]);
        break;
      case "white":
        appendStylesheet(styleSheets[4]);
        break;
      default:
        appendStylesheet(
          styleSheets[1],
          "(prefers-color-scheme: dark) and (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets[3],
          "(prefers-color-scheme: dark) and not (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets[2],
          "(prefers-color-scheme: light) and (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets[4],
          "((prefers-color-scheme: light) and not (prefers-contrast: no-preference)), print",
        );
        break;
    }
  }
}

function getLanguageQueryParam(): string | undefined {
  const url = new URL(window.location.href);
  let lang: string | null | undefined = url.searchParams.get("_LANG");
  if (lang === null) {
    lang = undefined;
  }
  return lang;
}

async function getLanguage(): Promise<string> {
  let queryLang = getLanguageQueryParam();
  let cookieLang = await getLanguageCookie();
  if (queryLang !== undefined) {
    await setLanguageCookie(queryLang);
    return queryLang;
  }
  if (cookieLang !== undefined) {
    return cookieLang;
  }
  return "auto";
}

async function setLanguageCookie(language: string): Promise<void> {
  const Cookies = await import("js-cookie");
  if (language !== "auto")
    Cookies.default.set("_LANG", language, {
      expires: 365,
      secure: true,
      sameSite: "Strict",
    });
  else Cookies.default.remove("_LANG");
}

async function setThemeCookie(theme: string): Promise<void> {
  const Cookies = await import("js-cookie");
  if (theme !== "auto")
    Cookies.default.set("_THEME", theme, {
      expires: 365,
      secure: true,
      sameSite: "Strict",
    });
  else Cookies.default.remove("_THEME");
}

async function updateLanguage(e: Event): Promise<void> {
  e.preventDefault();
  const lang = (e.target as HTMLSelectElement).value;
  await setLanguageCookie(lang);
  let uri = new URL(window.location.href);
  uri.searchParams.delete("_LANG");
  window.location.replace(uri);
}

async function updateTheme(e: Event): Promise<void> {
  e.preventDefault();
  const theme = (e.target as HTMLSelectElement).value;
  await setThemeCookie(theme);
  await applyTheme(theme);
}

export async function main() {
  let chosenLanguage = await getLanguage();
  let chosenTheme = await getTheme();
  let changeLanguage = document.getElementById("change-language")!!;
  let changeLanguageOptions = changeLanguage.getElementsByTagName("option")!!;
  for (let i = 0; i < changeLanguageOptions.length; i++) {
    changeLanguageOptions[i].selected =
      changeLanguageOptions[i].value == chosenLanguage;
  }
  changeLanguage.addEventListener("change", updateLanguage);
  let changeTheme = document.getElementById("change-theme")!!;
  let changeThemeOptions = changeTheme.getElementsByTagName("option")!!;
  for (let i = 0; i < changeThemeOptions.length; i++) {
    changeThemeOptions[i].selected = changeThemeOptions[i].value == chosenTheme;
  }
  changeTheme.addEventListener("change", updateTheme);
}

main();
