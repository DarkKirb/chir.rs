import { Stylesheets, fetchStylesheets } from "./api/style-files";

let styleSheets: Stylesheets | undefined = undefined;

function appendStylesheet(url: string, media?: string): void {
  let link = document.createElement("link");
  link.setAttribute("rel", "stylesheet");
  link.setAttribute("href", url);
  if (media !== undefined) {
    link.setAttribute("media", media);
  }
  document.head.appendChild(link);
}

export async function applyTheme(theme: string | undefined): Promise<void> {
  if (styleSheets === undefined) {
    styleSheets = await fetchStylesheets();
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
    appendStylesheet(styleSheets.baseStyle);
    switch (theme) {
      case "sunset":
        appendStylesheet(styleSheets.sunsetTheme);
        break;
      case "trans-rights":
        appendStylesheet(styleSheets.transRightsTheme);
        break;
      case "black":
        appendStylesheet(styleSheets.blackTheme);
        break;
      case "white":
        appendStylesheet(styleSheets.whiteTheme);
        break;
      default:
        appendStylesheet(
          styleSheets.sunsetTheme,
          "(prefers-color-scheme: dark) and (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets.blackTheme,
          "(prefers-color-scheme: dark) and not (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets.transRightsTheme,
          "(prefers-color-scheme: light) and (prefers-contrast: no-preference)",
        );
        appendStylesheet(
          styleSheets.whiteTheme,
          "((prefers-color-scheme: light) and not (prefers-contrast: no-preference)), print",
        );
        break;
    }
  }
}
