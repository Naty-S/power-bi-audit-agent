<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { locale, t } from '$lib/i18n';

	let { children } = $props();

	let lang_menu_open = $state(false);
  let scroll_y = $state(0);

  function change_lang(lang: string) {
    locale.set(lang);
    lang_menu_open = false;
  }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<svelte:window bind:scrollY={scroll_y} />

<nav class="fixed top-0 left-0 w-full z-50 transition-all duration-500 ease-[cubic-bezier(0.16,1,0.3,1)] 
  {scroll_y > 20 
    ? 'py-3 bg-shiori-dark/95 backdrop-blur-md border-b border-shiori-card shadow-lg' 
    : 'py-6 bg-transparent border-transparent'}">

  <div class="max-w-[1280px] mx-auto px-6 lg:px-12">
    <div class="flex items-center justify-between">
      
      <!-- Logo -->
      <div class="flex-shrink-0 flex items-center gap-3 cursor-pointer">
        <!-- Ícono placeholder (Robot/IA) -->
        <div class="w-10 h-10 bg-gradient-to-br from-shiori-turq to-shiori-light rounded-lg flex items-center justify-center text-shiori-dark font-bold text-xl">
					S
				</div>
        <span class="text-2xl font-display font-bold text-white tracking-wider">
					SHIO<span class="text-shiori-turq">RI</span>
				</span>
      </div>

      <!-- Desktop Menu -->
      <div class="hidden md:block">
        <div class="ml-10 flex items-baseline space-x-8">
          <a href="#home" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						{$t('nav.home')}
					</a>
          <a href="#benefits" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						Beneficios
					</a>
          <a href="#features" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						{$t('nav.features')}
					</a>
          <a href="#pricing" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						{$t('nav.pricing')}
					</a>
          <a href="#team" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						{$t('nav.team')}
					</a>
          <a href="#contact" class="text-gray-300 hover:text-shiori-turq px-3 py-2 rounded-md text-sm font-medium transition-colors">
						{$t('nav.contact')}
					</a>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex items-center gap-4">
        <!-- Language Dropdown -->
        <div class="relative">
          <button onclick={() => lang_menu_open = !lang_menu_open} class="text-gray-300 hover:text-shiori-turq flex items-center gap-1.5 text-sm font-medium transition-colors">
            {$locale === 'es' ? 'ES' : 'EN'} <span class="text-[10px]">▼</span>
          </button>
          {#if lang_menu_open}
            <div class="absolute right-0 mt-2 w-24 bg-shiori-card rounded-md shadow-lg py-1 border border-shiori-cyan">
              <button onclick={() => change_lang('es')} class="block w-full text-left px-4 py-2 text-sm text-gray-200 hover:bg-shiori-dark hover:text-white">
								Español
							</button>
              <button onclick={() => change_lang('en')} class="block w-full text-left px-4 py-2 text-sm text-gray-200 hover:bg-shiori-dark hover:text-white">
								English
							</button>
            </div>
          {/if}
        </div>

        <!-- Botón Demo -->
        <a href="/demo" class="bg-gradient-to-r from-shiori-light to-shiori-turq text-shiori-dark font-bold px-6 py-2.5 rounded-xl text-sm hover:scale-105 transition-transform shadow-[0_0_15px_rgba(99,212,195,0.4)]">
          {$t('nav.demo')}
        </a>
      </div>

    </div>
  </div>
</nav>

{@render children()}
