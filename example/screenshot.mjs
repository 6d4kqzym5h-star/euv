import puppeteer from 'puppeteer-core';

const CHROME_PATH = 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe';
const pages = [
  { name: 'home', url: 'http://127.0.0.1:3000/example/www/index.html#/' },
  { name: 'form', url: 'http://127.0.0.1:3000/example/www/index.html#/form' },
  { name: 'list', url: 'http://127.0.0.1:3000/example/www/index.html#/list' },
];

async function findChrome() {
  const fs = await import('fs');
  const paths = [
    'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
    'C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe',
    process.env.LOCALAPPDATA + '\\Google\\Chrome\\Application\\chrome.exe',
    'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
    'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe',
  ];
  for (const p of paths) {
    try { if (fs.existsSync(p)) return p; } catch {}
  }
  return null;
}

const chromePath = await findChrome();
if (!chromePath) {
  console.error('No Chrome/Edge found!');
  process.exit(1);
}
console.log('Using browser:', chromePath);

const browser = await puppeteer.launch({
  headless: true,
  executablePath: chromePath,
  args: ['--no-sandbox', '--disable-setuid-sandbox'],
});

const page = await browser.newPage();
await page.setViewport({ width: 1400, height: 900 });

for (const p of pages) {
  console.log(`Screenshotting ${p.name}...`);
  await page.goto(p.url, { waitUntil: 'networkidle0', timeout: 15000 });
  await new Promise(r => setTimeout(r, 2000));
  await page.screenshot({ path: `d:/code/euv/example/screenshot_${p.name}.png`, fullPage: false });
  console.log(`Saved screenshot_${p.name}.png`);
}

// Get DOM structure and computed styles for home page
await page.goto(pages[0].url, { waitUntil: 'networkidle0', timeout: 15000 });
await new Promise(r => setTimeout(r, 2000));

const analysis = await page.evaluate(() => {
  const result = {};

  // Root layout
  const rootDiv = document.querySelector('#app > div');
  if (rootDiv) {
    const cs = getComputedStyle(rootDiv);
    result.rootDiv = {
      display: cs.display,
      flexDirection: cs.flexDirection,
      width: cs.width,
      height: cs.height,
      children: rootDiv.children.length,
      childTags: Array.from(rootDiv.children).map(c => c.tagName + (c.id ? '#'+c.id : '') + '.' + c.className),
    };
  }

  // Nav
  const nav = document.querySelector('nav');
  if (nav) {
    const cs = getComputedStyle(nav);
    result.nav = {
      display: cs.display,
      flexDirection: cs.flexDirection,
      width: cs.width,
      height: cs.height,
      position: cs.position,
      children: nav.children.length,
      childTags: Array.from(nav.children).map(c => c.tagName + ': ' + getComputedStyle(c).display),
    };
  }

  // Main content
  const main = document.querySelector('main');
  if (main) {
    const cs = getComputedStyle(main);
    result.main = {
      display: cs.display,
      flex: cs.flex,
      padding: cs.padding,
      width: cs.width,
    };
  }

  // Check for extra wrapper divs (Fragment rendering issue)
  const allDivs = document.querySelectorAll('#app div');
  result.totalDivs = allDivs.length;
  result.divStyles = Array.from(allDivs).slice(0, 15).map(d => {
    const cs = getComputedStyle(d);
    return {
      display: cs.display,
      width: cs.width,
      height: cs.height,
      parent: d.parentElement?.tagName,
      style: d.getAttribute('style')?.substring(0, 80),
    };
  });

  return result;
});

console.log('\n=== Layout Analysis ===');
console.log(JSON.stringify(analysis, null, 2));

await browser.close();
console.log('Done!');
