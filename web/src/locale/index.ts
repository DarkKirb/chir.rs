import type { FluentBundle, FluentResource } from "@fluent/bundle";

const LANGUAGE_IDS: Record<string, string> = {
    'de': 'de-DE',
    'de-DE': 'de-DE',
    'en': 'en-US',
    'en-US': 'en-US',
    'en-IE': 'en-GB',
    'en-Sten-US': 'en-Sten-US',
    'en-Sten-US-display': 'en-Sten-US',
    'en-Sten-US-runes': 'en-Sten-US',
    'en-GB': 'en-GB',
    'fr': 'fr-FR',
    'fr-FR': 'fr-FR',
    'jbo': 'jbo-Latn-XX',
    'jbo-XX': 'jbo-Latn-XX',
    'jbo-Latn-XX': 'jbo-Latn-XX',
    'jbo-Zblr-XX': 'jbo-Zblr-XX',
    'nl': 'nl-NL',
    'nl-NL': 'nl-NL',
    'tok': 'tok-Latn-XX',
    'tok-XX': 'tok-Latn-XX',
    'tok-Emsi-XX': 'tok-Emsi-XX',
    'tok-Hani-XX': 'tok-Hani-XX',
    'tok-Latn-XX': 'tok-Latn-XX',
    'tok-Stln-XX': 'tok-Stln-XX',
};

let loadedLanguages: Record<string, FluentBundle> = {};

async function pickBundle(languageId: string): Promise<string> {
    switch(languageId) {
        case 'de': return (await import('../../../locales/de-DE/main.ftl')).default
        case 'de-DE': return (await import('../../../locales/de-DE/main.ftl')).default
        case 'en': return (await import('../../../locales/en-US/main.ftl')).default
        case 'en-US': return (await import('../../../locales/en-US/main.ftl')).default
        case 'en-IE': return (await import('../../../locales/en-GB/main.ftl')).default
        case 'en-Sten-US': return (await import('../../../locales/en-Sten-US/main.ftl')).default
        case 'en-Sten-US-display': return (await import('../../../locales/en-Sten-US/main.ftl')).default
        case 'en-Sten-US-runes': return (await import('../../../locales/en-Sten-US/main.ftl')).default
        case 'en-GB': return (await import('../../../locales/en-GB/main.ftl')).default
        case 'fr': return (await import('../../../locales/fr-FR/main.ftl')).default
        case 'fr-FR': return (await import('../../../locales/fr-FR/main.ftl')).default
        case 'jbo': return (await import('../../../locales/jbo-Latn-XX/main.ftl')).default
        case 'jbo-XX': return (await import('../../../locales/jbo-Latn-XX/main.ftl')).default
        case 'jbo-Latn-XX': return (await import('../../../locales/jbo-Latn-XX/main.ftl')).default
        case 'jbo-Zblr-XX': return (await import('../../../locales/jbo-Zblr-XX/main.ftl')).default
        case 'nl': return (await import('../../../locales/nl-NL/main.ftl')).default
        case 'nl-NL': return (await import('../../../locales/nl-NL/main.ftl')).default
        case 'tok': return (await import('../../../locales/tok-Latn-XX/main.ftl')).default
        case 'tok-XX': return (await import('../../../locales/tok-Latn-XX/main.ftl')).default
        case 'tok-Emsi-XX': return (await import('../../../locales/tok-Emsi-XX/main.ftl')).default
        case 'tok-Hani-XX': return (await import('../../../locales/tok-Hani-XX/main.ftl')).default
        case 'tok-Latn-XX': return (await import('../../../locales/tok-Latn-XX/main.ftl')).default
        case 'tok-Stln-XX': return (await import('../../../locales/tok-Stln-XX/main.ftl')).default
    }
    throw new Error("Unknown language: " + languageId);
}

async function loadLanguage(lang: string): Promise<FluentBundle> {
    if (loadedLanguages[lang]) {
        return loadedLanguages[lang];
    }

    const { FluentBundle, FluentResource } = await import("@fluent/bundle");
    const fluentUrl = await pickBundle(lang);
    const fluentFile = await fetch(fluentUrl).then(res => res.text());
    const fluentResource = new FluentResource(fluentFile);
    loadedLanguages[lang] = new FluentBundle(lang);
    loadedLanguages[lang].addResource(fluentResource);
    return loadedLanguages[lang];
}

function setRootLanguage(lang: string) {
    document.documentElement.lang = lang;
}

async function localize(langs: string[]) {
    let bundles: FluentBundle[] = [];
    for (const lang of langs) {
        bundles.push(await loadLanguage(LANGUAGE_IDS[lang]))
    }

    document.querySelectorAll('[data-l10n-id]').forEach((element) => {
        let l10nId = element.getAttribute('data-l10n-id') ?? "undefined";
        for (const bundle of bundles) {
            let message = bundle.getMessage(l10nId);
            if (message?.value) {
                let content = bundle.formatPattern(message.value, JSON.parse(element.getAttribute('data-l10n-args')?? '{}'))
                if(element.hasAttribute('data-l10n-property')) {
                    element.setAttribute(element.getAttribute('data-l10n-property')!, content);
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
    const Cookies = (await import('js-cookie')).default;
    const lang = Cookies.get('locale');
    if (lang) {
        yield lang;
    }

    yield* findBrowserLanguageIds();

    yield 'en-US';
}


async function localizeAll(): Promise<void> {
    const langs = []
    for await (const lang of getUserLanguageIds()) {
        langs.push(lang);
    }
    await localize(langs);
}

export async function handleLanguageChange(event: Event) {
    const Cookies = (await import('js-cookie')).default;
    let target = event.target as HTMLSelectElement;
    if (target.value === 'auto') {
        Cookies.remove('locale');
    } else {
        Cookies.set('locale', target.value, { expires: 365 });
    }
    await localizeAll();
}