<script lang="ts">
  import { t } from '$lib/i18n';
	import HeroDashboard from '$lib/HeroDashboard.svelte';
	import BenefitCard from '$lib/BenefitCard.svelte';

  let benefit = $state(0);

  // --- LÓGICA PARA EL CARRUSEL 3D (CLEARWAVE STYLE) ---
  let currentCenter = $state(2);
  const totalScreens = 5;
  
  // Imágenes mock para la App
  const screens = [
    { id: 0, src: 'https://via.placeholder.com/300x600/02122d/3885b3?text=Pantalla+1' },
    { id: 1, src: 'https://via.placeholder.com/300x600/02122d/63d4c3?text=Pantalla+2' },
    { id: 2, src: 'https://via.placeholder.com/300x600/042b59/63d4c3?text=Dashboard' },
    { id: 3, src: 'https://via.placeholder.com/300x600/02122d/14625d?text=Pantalla+4' },
    { id: 4, src: 'https://via.placeholder.com/300x600/042b59/3885b3?text=Pantalla+5' }
  ];

  function next() { currentCenter = (currentCenter + 1) % totalScreens; }
  function prev() { currentCenter = (currentCenter - 1 + totalScreens) % totalScreens; }
  function goTo(index: number) { currentCenter = index; }

  // Calcula la posición relativa al centro
  function getPosition(index: number) {
    let offset = index - currentCenter;
    if (offset > Math.floor(totalScreens / 2)) offset -= totalScreens;
    if (offset < -Math.floor(totalScreens / 2)) offset += totalScreens;
    return offset;
  }

  // Genera el estilo 3D basado en la posición
  function getCardStyle(offset: number) {
    const absOffset = Math.abs(offset);
    const sign = Math.sign(offset);

    if (offset === 0) {
      return 'transform: translateX(0) scale(1) rotateY(0deg); z-index: 50; opacity: 1;';
    }
    if (absOffset === 1) {
      return `transform: translateX(${sign * 50}%) scale(0.85) rotateY(${-sign * 25}deg); z-index: 40; opacity: 0.8;`;
    }
    if (absOffset === 2) {
      return `transform: translateX(${sign * 90}%) scale(0.7) rotateY(${-sign * 45}deg); z-index: 30; opacity: 0.4;`;
    }
    return `transform: translateX(${sign * 100}%) scale(0.5); z-index: 10; opacity: 0; pointer-events: none;`;
  }
</script>

<section id="home" class="relative min-h-[95vh] flex items-center pt-28 pb-16 overflow-hidden">
  <!-- Fondos decorativos (NovaPay style) -->
  <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top_left,_rgba(56,133,179,0.15)_0%,_transparent_50%)] pointer-events-none"></div>
  <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-[radial-gradient(circle,_rgba(99,212,195,0.08)_0%,_transparent_60%)] -translate-y-1/4 translate-x-1/4 rounded-full pointer-events-none"></div>
  
  <div class="max-w-[1280px] mx-auto px-6 lg:px-12 relative z-10 w-full">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
      
      <!-- Columna Izquierda: Textos -->
      <div>
        <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-shiori-turq/30 bg-shiori-turq/10 text-shiori-turq mb-8 text-xs font-medium tracking-wide">
          <span class="w-1.5 h-1.5 rounded-full bg-shiori-turq animate-pulse"></span>
          Con análisis de IA integrado
        </div>
        
        <h1 class="text-4xl md:text-5xl lg:text-[56px] font-display font-bold mb-6 leading-[1.1] tracking-tight">
          El guardián de tu <br>
          <span class="text-shiori-turq">integridad financiera.</span>
        </h1>
        
        <p class="text-lg md:text-xl text-gray-300 font-light italic mb-6 border-l-2 border-shiori-turq pl-4">
          "{$t('hero.subtitle')}"
        </p>

        <p class="text-base text-gray-400 mb-10 max-w-lg leading-relaxed">
          Objetivo: {$t('hero.obj')}
          Pilares: Integración de datos, análisis de big data, detección de incidentes en tiempo real, presentación de resultados.
        </p>

        <div class="flex items-center gap-4">
          <a href="#contact" class="bg-shiori-turq text-shiori-dark font-bold px-8 py-3.5 rounded-xl text-sm hover:bg-white transition-all duration-300 flex items-center gap-2">
            Comenzar gratis <span class="text-lg leading-none">→</span>
          </a>
          <a href="#features" class="text-gray-300 border border-shiori-card bg-transparent hover:border-shiori-turq hover:text-shiori-turq px-8 py-3.5 rounded-xl text-sm font-medium transition-colors">
            Ver en acción
          </a>
        </div>
      </div>

      <!-- Columna Derecha: Dashboard Mockup -->
       <HeroDashboard />

    </div>
  </div>
</section>

<!-- BENEFICIOS / FEATURES STICKY STACK -->
<section id="benefits" class="py-24 max-w-[1280px] mx-auto px-6 lg:px-12">
  <!-- Header de la sección -->
  <div class="text-center max-w-3xl mx-auto mb-20">
    <div class="inline-block text-[11px] font-semibold tracking-[0.12em] uppercase text-shiori-turq mb-4">
      Plataforma Integral
    </div>
    <h2 class="font-display text-4xl md:text-5xl font-bold tracking-tight text-white mb-4">
      Todo lo que tú empresa necesita
    </h2>
    <p class="text-lg text-gray-400 font-light">
      Auditoría Inteligente en tiempo real
    </p>
  </div>

  <!-- Layout Sticky -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-20 items-start">
    
    <!-- Lado Izquierdo: Cards -->
    <div class="flex flex-col gap-4">
      
      <!-- Card 1 -->
      <!-- <BenefitCard bind:benefit onclick={() => benefit = 0} /> -->
      <button class="text-left border border-shiori-card hover:translate-x-2 {benefit === 0 ? 'bg-shiori-card/40 border-shiori-turq/50' : 'bg-shiori-card/20'} rounded-2xl p-7 cursor-pointer transition-all duration-500 relative overflow-hidden group shadow-lg"
        onclick={() => benefit = 0}
      >
        <!-- Left line -->
        <div class="absolute left-0 top-0 bottom-0 w-1 bg-shiori-turq transition-opacity duration-500 {benefit === 0 ? 'opacity-100' : 'opacity-0'}"></div>
        
        <!-- Shield Icon -->
        <div class="w-11 h-11 bg-shiori-turq/10 border border-shiori-turq/20 rounded-xl flex items-center justify-center mb-4 transition-colors {benefit === 0 ? 'bg-shiori-turq/20 border-shiori-turq/50' : ''}">
          <svg class="w-5 h-5 text-shiori-turq" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
          </svg>
        </div>
        
        <h3 class="font-display text-[18px] font-bold text-white mb-2">{$t(`benefits.0.title`)}</h3>
        <p class="text-sm text-gray-400 leading-[1.65] font-light">{$t(`benefits.0.text`)}</p>
        
        <div class="flex gap-2 mt-4 flex-wrap">
          {#each $t(`benefits.0.tags`) as tag }
            <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">
              {tag}
            </span>
          {/each}
        </div>
      </button>

      <!-- Card 2 -->
      <button class="text-left border border-shiori-card hover:translate-x-2 {benefit === 1 ? 'bg-shiori-card/40 border-shiori-turq/50' : 'bg-shiori-card/20'} rounded-2xl p-7 cursor-pointer transition-all duration-500 relative overflow-hidden group shadow-lg"
        onclick={() => benefit = 1}
      >
        <div class="absolute left-0 top-0 bottom-0 w-1 bg-shiori-turq transition-opacity duration-500 {benefit === 1 ? 'opacity-100' : 'opacity-0'}"></div>
        
        <div class="w-11 h-11 bg-shiori-turq/10 border border-shiori-turq/20 rounded-xl flex items-center justify-center mb-4 transition-colors {benefit === 1 ? 'bg-shiori-turq/20 border-shiori-turq/50' : ''}">
          <!-- Icono Check/Cumplimiento -->
          <svg class="w-5 h-5 text-shiori-turq" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24"><path d="M9 11l3 3L22 4"/><path d="M21 12v7a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2h11"/></svg>
        </div>
        
        <h3 class="font-display text-[18px] font-bold text-white mb-2">Cumplimiento de la normativa técnica</h3>
        <p class="text-sm text-gray-400 leading-[1.65] font-light">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit.
        </p>
        
        <div class="flex gap-2 mt-4 flex-wrap">
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">ISO 27001</span>
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">Auditoría continua</span>
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">Reportes legales</span>
        </div>
      </button>

      <!-- Card 3 -->
      <button class="text-left border border-shiori-card hover:translate-x-2 {benefit === 2 ? 'bg-shiori-card/40 border-shiori-turq/50' : 'bg-shiori-card/20'} rounded-2xl p-7 cursor-pointer transition-all duration-500 relative overflow-hidden group shadow-lg"
        onclick={() => benefit = 2}
      >
        <div class="absolute left-0 top-0 bottom-0 w-1 shiori-turq transition-opacity duration-500 {benefit === 2 ? 'opacity-100' : 'opacity-0'}"></div>
        
        <div class="w-11 h-11 bg-shiori-turq/10 border border-shiori-turq/20 rounded-xl flex items-center justify-center mb-4 transition-colors {benefit === 2 ? 'bg-shiori-turq/20 border-shiori-turq/50' : ''}">
          <!-- Icono Gráfico/Dashboard -->
          <svg class="w-5 h-5 text-shiori-turq" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
            <rect x="3" y="3" width="18" height="18" rx="2"/>
            <path d="M3 9h18M9 21V9"/>
          </svg>
        </div>
        
        <h3 class="font-display text-[18px] font-bold text-white mb-2">Visualización digital de la salud financiera y operativa del negocio</h3>
        <p class="text-sm text-gray-400 leading-[1.65] font-light">
          Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nullam do magna sit amet dictum. Aenean lacinia bibendum nulla sed consectetur.
        </p>
        
        <div class="flex gap-2 mt-4 flex-wrap">
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">Tiempo real</span>
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">Dashboards interactivos</span>
          <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">Métricas clave</span>
        </div>
      </button>
    </div>

    <!-- Lado Derecho: Panel Sticky (Tu componente Mockup) -->
    <div class="sticky top-32 relative z-10 w-full rounded-2xl overflow-hidden shadow-2xl">
      <HeroDashboard />
    </div>

  </div>
</section>

<!-- FEATURES SECTION (CLEARWAVE CAROUSEL + MOBAPP CARDS) -->
<section id="features" class="py-24 bg-shiori-dark overflow-hidden border-t border-shiori-card">
  
  <!-- Título principal -->
  <div class="max-w-[1280px] mx-auto px-6 lg:px-12 text-center mb-16">
    <h2 class="text-4xl md:text-5xl font-display font-bold text-white tracking-tight">
      Auditoría TI en tu bolsillo
    </h2>
  </div>

  <!-- PARTE 1: CARRUSEL 3D MÓVIL (Clearwave Style) -->
  <div class="relative h-[480px] md:h-[580px] w-full max-w-6xl mx-auto flex items-center justify-center mb-24" style="perspective: 1200px;">
    
    <!-- Contenedor de las tarjetas -->
    <div class="relative w-full h-full flex items-center justify-center" style="transform-style: preserve-3d;">
      {#each screens as screen, i}
        {@const offset = getPosition(i)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div 
          onclick={() => goTo(i)}
          class="absolute w-[220px] md:w-[280px] h-[450px] md:h-[560px] rounded-[32px] overflow-hidden shadow-[0_20px_50px_rgba(2,18,45,0.8)] transition-all duration-700 ease-[cubic-bezier(0.16,1,0.3,1)] cursor-pointer border-[6px] border-[#0a182e] bg-[#0a182e]"
          style={getCardStyle(offset)}
        >
          <img src={screen.src} alt="App screen {i}" class="w-full h-full object-cover rounded-[24px]" />
        </div>
      {/each}
    </div>

    <!-- Controles del carrusel -->
    <div class="absolute -bottom-10 left-0 right-0 flex justify-center items-center gap-6">
      <button onclick={prev} class="w-10 h-10 rounded-full border border-shiori-card flex items-center justify-center text-shiori-turq hover:bg-shiori-card transition-colors">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/></svg>
      </button>
      
      <div class="flex gap-3">
        {#each screens as _, i}
          <button 
            onclick={() => goTo(i)} 
            class="h-2 rounded-full transition-all duration-300 {currentCenter === i ? 'w-6 bg-shiori-turq' : 'w-2 bg-shiori-card hover:bg-shiori-light/50'}"
            aria-label="Ir a pantalla {i + 1}"
          ></button>
        {/each}
      </div>

      <button onclick={next} class="w-10 h-10 rounded-full border border-shiori-card flex items-center justify-center text-shiori-turq hover:bg-shiori-card transition-colors">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
      </button>
    </div>
  </div>

  <!-- PARTE 2: TARJETAS DE CARACTERÍSTICAS (MobApp Style) -->
  <div class="max-w-[1280px] mx-auto px-6 lg:px-12 grid grid-cols-1 md:grid-cols-3 gap-8 mt-16 relative">
    
    <!-- Tarjeta 1: API Robusta -->
    <div class="bg-shiori-card/20 border border-shiori-card p-8 rounded-2xl hover:-translate-y-2 hover:border-shiori-turq/50 hover:bg-shiori-card/40 transition-all duration-300 group shadow-lg">
      <div class="flex items-start gap-5">
        <div class="text-shiori-turq mt-1 group-hover:scale-110 transition-transform">
          <!-- Icono API / Base de datos -->
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
          </svg>
        </div>
        <div>
          <h4 class="text-xl font-bold text-white mb-3">API Robusta</h4>
          <p class="text-sm text-gray-400 leading-relaxed">
            Ingesta de datos ininterrumpida y segura.
          </p>
        </div>
      </div>
    </div>

    <!-- Tarjeta 2: Servicios de IA -->
    <div class="bg-shiori-card/20 border border-shiori-card p-8 rounded-2xl hover:-translate-y-2 hover:border-shiori-turq/50 hover:bg-shiori-card/40 transition-all duration-300 group shadow-lg">
      <div class="flex items-start gap-5">
        <div class="text-shiori-turq mt-1 group-hover:scale-110 transition-transform">
          <!-- Icono Inteligencia Artificial / Chip -->
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"/></svg>
        </div>
        <div>
          <h4 class="text-xl font-bold text-white mb-3">Servicios de IA</h4>
          <p class="text-sm text-gray-400 leading-relaxed">
            El motor de detección de patrones complejos y anomalías.
          </p>
        </div>
      </div>
    </div>

    <!-- Tarjeta 3: Power BI -->
    <div class="bg-shiori-card/20 border border-shiori-card p-8 rounded-2xl hover:-translate-y-2 hover:border-shiori-turq/50 hover:bg-shiori-card/40 transition-all duration-300 group shadow-lg">
      <div class="flex items-start gap-5">
        <div class="text-shiori-turq mt-1 group-hover:scale-110 transition-transform">
          <!-- Icono Power BI / Gráficas -->
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/></svg>
        </div>
        <div>
          <h4 class="text-xl font-bold text-white mb-3">Power BI</h4>
          <p class="text-sm text-gray-400 leading-relaxed">
            Visualización de riesgos en paneles dinámicos y accionables.
          </p>
        </div>
      </div>
    </div>

  </div>

  <!-- Texto de resumen inferior -->
  <div class="text-center mt-12 mb-8 text-gray-300 font-light text-lg max-w-3xl mx-auto px-6">
    Una infraestructura diseñada para integrarse sin fricción en los sistemas financieros existentes.
  </div>
</section>

<!-- PRICING SECTION -->
<section id="pricing" class="py-24 max-w-[1280px] mx-auto px-6 lg:px-12">
  
  <!-- Header de Precios -->
  <div class="text-center max-w-3xl mx-auto mb-16 md:mb-20">
    <div class="inline-block text-[11px] font-semibold tracking-[0.12em] uppercase text-shiori-turq mb-4">
      Precios Simples
    </div>
    <h2 class="font-display text-4xl md:text-5xl font-bold tracking-tight text-white mb-4">
      Transparente, sin sorpresas
    </h2>
    <p class="text-lg text-gray-400 font-light">
      Empieza a escalar cuando estés listo. Sin tarifas ocultas, nunca.
    </p>
  </div>
  
  <!-- Grid de Tarjetas -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 lg:gap-8 items-center max-w-6xl mx-auto">
    
    <!-- PLAN BÁSICO -->
    <div class="bg-shiori-card/20 border border-shiori-cyan/40 rounded-3xl p-8 lg:p-10 hover:border-shiori-turq/60 hover:bg-shiori-card/30 transition-all duration-300 flex flex-col h-full shadow-lg">
      <h3 class="text-[13px] font-bold tracking-[0.1em] uppercase text-gray-400 mb-6">Plan Básico</h3>
      <div class="flex items-baseline gap-1 mb-2">
        <span class="text-5xl font-display font-bold text-white tracking-tight">150$</span>
        <span class="text-sm text-gray-400">/ al mes</span>
      </div>
      
      <p class="text-sm px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq mb-8">Micro-PyMES</p>
      
      <div class="w-full h-px bg-shiori-cyan/30 mb-8"></div>
      
      <ul class="space-y-4 text-sm text-gray-300 flex-1 mb-10">
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Hasta 3.000 transacciones</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Análisis del 100% de la data</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Reporte básico de anomalías</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>5 Dashboards</span>
        </li>
      </ul>
      
      <button class="w-full py-3.5 rounded-xl border-2 border-shiori-cyan/50 text-white font-semibold hover:border-shiori-turq hover:text-shiori-turq transition-colors">
        Comenzar
      </button>
    </div>

    <!-- PLAN CRECIMIENTO (DESTACADO) -->
    <div class="relative bg-shiori-card border-2 border-shiori-turq rounded-3xl p-8 lg:p-10 transform md:-translate-y-6 shadow-[0_15px_50px_rgba(99,212,195,0.15)] flex flex-col h-full z-10">
      <!-- Badge Recomendado -->
      <div class="absolute -top-3.5 left-1/2 -translate-x-1/2 bg-shiori-turq text-shiori-dark text-[11px] font-bold uppercase tracking-[0.1em] py-1.5 px-4 rounded-full whitespace-nowrap">
        Más Popular
      </div>
      
      <h3 class="text-[13px] font-bold tracking-[0.1em] uppercase text-white mb-6">Plan Crecimiento</h3>
      <div class="flex items-baseline gap-1 mb-2">
        <span class="text-5xl font-display font-bold text-white tracking-tight">500$</span>
        <span class="text-sm text-gray-300">/ al mes</span>
      </div>
      
      <p class="text-sm px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq mb-8">PyMES</p>
      
      <div class="w-full h-px bg-shiori-light/30 mb-8"></div>
      
      <ul class="space-y-4 text-sm text-white flex-1 mb-10">
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Transacciones Ilimitadas</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Análisis del 100% de la data</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Alerta en tiempo real</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Integración con 3 ERP's</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Dashboards ilimitados</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Certificado Digital de Salud Financiera y Operativa del Negocio</span>
        </li>
      </ul>
      
      <button class="w-full py-3.5 rounded-xl bg-shiori-turq text-shiori-dark font-bold hover:bg-shiori-light hover:text-white transition-colors shadow-[0_6px_20px_rgba(99,212,195,0.3)]">
        Prueba de 14 días
      </button>
    </div>

    <!-- PAGO POR USO -->
    <div class="bg-shiori-card/20 border border-shiori-cyan/40 rounded-3xl p-8 lg:p-10 hover:border-shiori-turq/60 hover:bg-shiori-card/30 transition-all duration-300 flex flex-col h-full shadow-lg">
      <h3 class="text-[13px] font-bold tracking-[0.1em] uppercase text-gray-400 mb-6">Pago por uso</h3>
      <div class="flex items-baseline gap-1 mb-2">
        <span class="text-5xl font-display font-bold text-white tracking-tight">30$</span>
        <span class="text-sm text-gray-400">/ la hora</span>
      </div>
      
      <p class="text-sm px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq mb-8">Fintech / Plataformas</p>
      
      <div class="w-full h-px bg-shiori-cyan/30 mb-8"></div>
      
      <ul class="space-y-4 text-sm text-gray-300 flex-1 mb-10">
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Soporte Técnico</span>
        </li>
        <li class="flex items-start gap-3">
          <div class="w-5 h-5 rounded-full bg-shiori-turq/10 border border-shiori-turq/30 flex items-center justify-center flex-shrink-0 mt-0.5">
            <svg class="w-3 h-3 text-shiori-turq" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="3">
              <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <span>Premium por uso</span>
        </li>
      </ul>
      
      <button class="w-full py-3.5 rounded-xl border-2 border-shiori-cyan/50 text-white font-semibold hover:border-shiori-turq hover:text-shiori-turq transition-colors">
        Contactar ventas
      </button>
    </div>

  </div>

  <!-- Nota inferior -->
  <div class="text-center mt-12">
    <p class="text-sm text-shiori-turq font-medium bg-shiori-turq/10 inline-block px-4 py-2 rounded-full border border-shiori-turq/20">
      🎉 20% de descuento en la primera suscripción del plan seleccionado.
    </p>
  </div>
</section>

<!-- ============================================== -->
<!-- SECCIÓN ESPECIALISTAS (Inspirada en NovaPay/Clearwave) -->
<!-- ============================================== -->
<section id="team" class="py-24 max-w-[1280px] mx-auto px-6 lg:px-12 border-t border-shiori-card/50">
  <div class="text-center max-w-3xl mx-auto mb-16">
    <div class="inline-block text-[11px] font-semibold tracking-[0.12em] uppercase text-shiori-turq mb-4">
      Nuestro Equipo
    </div>
    <h2 class="font-display text-4xl md:text-5xl font-bold tracking-tight text-white mb-4">
      Especialistas
    </h2>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
    <!-- Especialista 1: Aura -->
    <div class="bg-shiori-card/20 border border-shiori-cyan/40 p-6 md:p-8 rounded-3xl hover:border-shiori-turq/60 hover:bg-shiori-card/30 transition-all duration-300 flex items-center gap-6 group shadow-lg">
      <div class="w-20 h-20 md:w-24 md:h-24 rounded-full bg-shiori-dark border-2 border-shiori-turq/30 overflow-hidden flex-shrink-0 group-hover:border-shiori-turq transition-colors shadow-[0_0_15px_rgba(99,212,195,0.2)]">
        <!-- Placeholder para la foto. Reemplazar src con la ruta real de la imagen -->
        <img src="https://via.placeholder.com/150/042b59/63d4c3?text=AG" alt="Lcda. Aura Gomes" class="w-full h-full object-cover" />
      </div>
      <div>
        <h3 class="text-xl font-bold text-white mb-1">Lcda. Aura Gomes</h3>
        <p class="text-sm text-shiori-turq font-medium tracking-wide">CEO</p>
      </div>
    </div>

    <!-- Especialista 2: Natascha -->
    <div class="bg-shiori-card/20 border border-shiori-cyan/40 p-6 md:p-8 rounded-3xl hover:border-shiori-turq/60 hover:bg-shiori-card/30 transition-all duration-300 flex items-center gap-6 group shadow-lg">
      <div class="w-20 h-20 md:w-24 md:h-24 rounded-full bg-shiori-dark border-2 border-shiori-turq/30 overflow-hidden flex-shrink-0 group-hover:border-shiori-turq transition-colors shadow-[0_0_15px_rgba(99,212,195,0.2)]">
        <!-- Placeholder para la foto. Reemplazar src con la ruta real de la imagen -->
        <img src="https://via.placeholder.com/150/042b59/3885b3?text=NG" alt="Ing. Natascha Gamboa" class="w-full h-full object-cover" />
      </div>
      <div>
        <h3 class="text-xl font-bold text-white mb-1">Ing. Natascha Gamboa</h3>
        <p class="text-sm text-shiori-light font-medium tracking-wide">Desarrolladora</p>
      </div>
    </div>
  </div>
</section>

<!-- ============================================== -->
<!-- SECCIÓN CONTACTO (Estilo MobApp adaptado a Shiori) -->
<!-- ============================================== -->
<footer id="contact" class="py-10 bg-shiori-dark border-t border-shiori-cyan/30 mt-12">
  <div class="max-w-[1280px] mx-auto px-6 lg:px-12">
    <div class="flex flex-col lg:flex-row items-center justify-between gap-8">

      <!-- Información Izquierda (Ubicación, Correo, Teléfono) -->
      <div class="flex flex-col lg:flex-row items-center gap-6 lg:gap-10 text-gray-400 text-sm font-light">
        
        <!-- Dirección -->
        <div class="flex items-center gap-2.5">
          <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.243-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
          <span>Chacao, Caracas-Venezuela</span>
        </div>
        
        <!-- Agrupación Correo y Teléfono -->
        <div class="flex flex-col sm:flex-row items-center gap-6 lg:gap-10">
          <!-- Correo -->
          <div class="flex items-center gap-2.5">
            <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
            <a href="mailto:auditoriatifshiori@gmail.com" class="text-shiori-turq hover:text-white transition-colors">auditoriatifshiori@gmail.com</a>
          </div>
          
          <!-- Teléfono -->
          <div class="flex items-center gap-2.5">
            <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
            <a href="tel:+584242995994" class="text-shiori-turq hover:text-white transition-colors">+58 424-2995994</a>
          </div>
        </div>

      </div>

      <!-- Redes Sociales Derecha (Estilo botones MobApp adaptados) -->
      <div class="flex items-center gap-3">
        <!-- Facebook -->
        <a href="#" aria-label="Facebook" class="w-10 h-10 rounded-xl bg-shiori-card/30 border border-shiori-cyan/40 flex items-center justify-center text-gray-400 hover:bg-shiori-turq hover:text-shiori-dark hover:border-shiori-turq transition-all duration-300">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M22 12c0-5.523-4.477-10-10-10S2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12z"/></svg>
        </a>
        <!-- Twitter / X -->
        <a href="#" aria-label="Twitter" class="w-10 h-10 rounded-xl bg-shiori-card/30 border border-shiori-cyan/40 flex items-center justify-center text-gray-400 hover:bg-shiori-turq hover:text-shiori-dark hover:border-shiori-turq transition-all duration-300">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-4.714-6.231-5.401 6.231H2.744l7.73-8.835L1.254 2.25H8.08l4.253 5.622zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
        </a>
        <!-- Instagram -->
        <a href="#" aria-label="Instagram" class="w-10 h-10 rounded-xl bg-shiori-card/30 border border-shiori-cyan/40 flex items-center justify-center text-gray-400 hover:bg-shiori-turq hover:text-shiori-dark hover:border-shiori-turq transition-all duration-300">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12.315 2c2.43 0 2.784.013 3.808.06 1.064.049 1.791.218 2.427.465a4.902 4.902 0 011.772 1.153 4.902 4.902 0 011.153 1.772c.247.636.416 1.363.465 2.427.048 1.067.06 1.407.06 4.123v.08c0 2.643-.012 2.987-.06 4.043-.049 1.064-.218 1.791-.465 2.427a4.902 4.902 0 01-1.153 1.772 4.902 4.902 0 01-1.772 1.153c-.636.247-1.363.416-2.427.465-1.067.048-1.407.06-4.123.06h-.08c-2.643 0-2.987-.012-4.043-.06-1.064-.049-1.791-.218-2.427-.465a4.902 4.902 0 01-1.772-1.153 4.902 4.902 0 01-1.153-1.772c-.247-.636-.416-1.363-.465-2.427-.047-1.024-.06-1.379-.06-3.808v-.63c0-2.43.013-2.784.06-3.808.049-1.064.218-1.791.465-2.427a4.902 4.902 0 011.153-1.772A4.902 4.902 0 015.45 2.525c.636-.247 1.363-.416 2.427-.465C8.901 2.013 9.256 2 11.685 2h.63zm-.081 1.802h-.468c-2.456 0-2.784.011-3.807.058-.975.045-1.504.207-1.857.344-.467.182-.8.398-1.15.748-.35.35-.566.683-.748 1.15-.137.353-.3.882-.344 1.857-.047 1.023-.058 1.351-.058 3.807v.468c0 2.456.011 2.784.058 3.807.045.975.207 1.504.344 1.857.182.466.399.8.748 1.15.35.35.683.566 1.15.748.353.137.882.3 1.857.344 1.054.048 1.37.058 4.041.058h.08c2.597 0 2.917-.01 3.96-.058.976-.045 1.505-.207 1.858-.344.466-.182.8-.398 1.15-.748.35-.35.566-.683.748-1.15.137-.353.3-.882.344-1.857.048-1.055.058-1.37.058-4.041v-.08c0-2.597-.01-2.917-.058-3.96-.045-.976-.207-1.505-.344-1.858a3.097 3.097 0 00-.748-1.15 3.098 3.098 0 00-1.15-.748c-.353-.137-.882-.3-1.857-.344-1.023-.047-1.351-.058-3.807-.058zM12 6.865a5.135 5.135 0 110 10.27 5.135 5.135 0 010-10.27zm0 1.802a3.333 3.333 0 100 6.666 3.333 3.333 0 000-6.666zm5.338-3.205a1.2 1.2 0 110 2.4 1.2 1.2 0 010-2.4z" clip-rule="evenodd"/></svg>
        </a>
      </div>

    </div>
  </div>
</footer>
