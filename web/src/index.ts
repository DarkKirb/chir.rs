import "./style.scss";

async function onLanguageChange(event: Event) {
  let internationalizationHandler = await import("./locale");
  await internationalizationHandler.handleLanguageChange(event);
}

async function onThemeChange(event: Event) {
  let themeHandler = await import("./theme");
  themeHandler.handleThemeChange(event);
}

async function main() {
  document
    .getElementById("change-language")
    ?.addEventListener("change", onLanguageChange);
  document
    .getElementById("change-theme")
    ?.addEventListener("change", onThemeChange);
  // Following this we no longer need the submit settings button
  //document.getElementById("settings-submit-button")?.remove();
}

addEventListener("DOMContentLoaded", main);
