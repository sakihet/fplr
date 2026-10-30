// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://sakihet.github.io',
	base: '/fplr',
	integrations: [
		starlight({
			title: 'fplr',
			description: 'A CLI for Fantasy Premier League in Rust',
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/sakihet/fplr' },
				{ icon: 'seti:rust', label: 'crates.io', href: 'https://crates.io/crates/fplr' },
			],
			editLink: {
				baseUrl: 'https://github.com/sakihet/fplr/edit/main/site/',
			},
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Installation', slug: 'getting-started/installation' },
						{ label: 'Configuration', slug: 'getting-started/configuration' },
					],
				},
				{
					label: 'Guides',
					items: [
						{ label: 'Shell Completion', slug: 'guides/shell-completion' },
						{ label: 'Color Output', slug: 'guides/color-output' },
					],
				},
				{
					label: 'Reference',
					items: [{ autogenerate: { directory: 'reference' } }],
				},
			],
		}),
	],
});
