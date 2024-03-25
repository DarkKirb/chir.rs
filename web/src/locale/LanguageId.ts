function toTitleCase(str: string): string {
  return str.replace(/\w\S*/g, function (txt) {
    return txt.charAt(0).toUpperCase() + txt.substring(1).toLowerCase();
  });
}

export class LanguageId {
  public language: string;
  public script: string | null;
  public region: string | null;
  public variant: string[];

  constructor(langId: string) {
    const parts = langId.split(/[-_]/);
    if (parts[0].match(/([a-zA-Z]{2,3}|[a-zA-Z]{5,8})/) === null) {
      throw new Error(`Invalid language id: ${langId}`);
    }
    this.language = parts[0].toLowerCase();
    parts.shift();
    if (parts[0].match(/[a-zA-Z]{4}/)) {
      this.script = toTitleCase(parts[0]);
      parts.shift();
    } else {
      this.script = null;
    }
    if (parts[0].match(/([a-zA-Z]{2}|[0-9]{3})/)) {
      this.region = parts[0].toUpperCase();
      parts.shift();
    } else {
      this.region = null;
    }
    if (parts.length > 0) {
      this.variant = parts.map((part) => part.toLowerCase());
    } else {
      this.variant = [];
    }
  }

  public toString(): string {
    return `${this.language}${this.script ? `-${this.script}` : ""}${this.region ? `-${this.region}` : ""}${this.variant.length > 0 ? `-${this.variant.join("-")}` : ""}`;
  }

  public removeLeastSignificantComponent(): boolean {
    if (this.variant.length > 0) {
      this.variant.pop();
      return true;
    } else if (this.region) {
      this.region = null;
      return true;
    } else if (this.script) {
      this.script = null;
      return true;
    }
    return false;
  }
}
