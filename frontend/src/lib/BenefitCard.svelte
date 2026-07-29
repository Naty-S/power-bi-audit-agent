<script lang="ts">
	import { t } from "./i18n";

  let {
    benefit = $bindable(0),
    // active = $bindable(false),
    onclick
  } = $props();
</script>


<button 
  class="text-left border border-shiori-cyan hover:translate-x-2 {benefit === 0 ? 'bg-shiori-turq/10' : 'bg-shiori-card/50'} rounded-2xl p-7 cursor-pointer transition-all duration-500 relative overflow-hidden group"
  {onclick}
>
  <!-- Left line -->
  <div class="absolute left-0 top-0 bottom-0 w-1 bg-shiori-turq transition-opacity duration-500 {benefit === 0 ? 'opacity-100' : 'opacity-0'}"></div>
  
  <!-- Shield Icon -->
  <div class="w-11 h-11 bg-shiori-turq/10 border border-shiori-turq/20 rounded-xl flex items-center justify-center mb-4 transition-colors {benefit === 0 ? 'bg-shiori-turq/20' : ''}">
    <svg class="w-5 h-5 text-shiori-turq" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
      <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
    </svg>
  </div>
  
  <h3 class="font-display text-[18px] font-bold text-white mb-2">{$t(`benefits.${benefit}.title`)}</h3>
  <p class="text-sm text-gray-400 leading-[1.65] font-light">{$t(`benefits.${benefit}.text`)}</p>
  
  <div class="flex gap-2 mt-4 flex-wrap">
    {#each $t(`benefits.${benefit}.tags`) as tag }
      <span class="text-[11px] font-medium px-2.5 py-1 rounded-full bg-shiori-turq/10 border border-shiori-turq/20 text-shiori-turq">
        {tag}
      </span>
    {/each}
  </div>
</button>
