import Cookies from "js-cookie";

function setTheme(theme: string) {
  if (theme === "auto") {
    Cookies.remove("theme");
    delete document.documentElement.dataset.theme;
  } else {
    Cookies.set("theme", theme, { expires: 365 });
    document.documentElement.dataset.theme = theme;
  }
}

export function handleThemeChange(event: Event) {
  let target = event.target as HTMLSelectElement;
  setTheme(target.value);
}
