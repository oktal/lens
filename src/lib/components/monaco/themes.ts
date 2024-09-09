import monaco from '$lib/monaco';

const exportedThemes = Object.fromEntries(
  Object.entries(import.meta.glob('./themes/*.json', { eager: true, query: '?raw', import: 'default' })).map(([k, v]) => [
    k.toLowerCase().split('/').reverse()[0].slice(0, -'.json'.length).replaceAll(' ', '-'),
    v
  ])
);

const nativeThemes = ['vs', 'vs-dark', 'hc-black'];

const themeNames: string[] = [...Object.keys(exportedThemes), ...nativeThemes].sort(
  (a, b) => a.localeCompare(b)
);

function setTheme(theme: string) {
  if (themeNames.includes(theme))
    monaco.editor.setTheme(theme);
}

function defineThemes() {
  Object.entries(exportedThemes).forEach(([name, theme]) => {
    const themeData = JSON.parse(theme as any);
    monaco.editor.defineTheme(name, themeData as any);
  });
}

export { setTheme, defineThemes, themeNames };
