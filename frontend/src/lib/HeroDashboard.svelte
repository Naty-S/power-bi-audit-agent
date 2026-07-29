<div class="relative w-full max-w-lg mx-auto lg:ml-auto perspective-1000">
  
  <!-- Tarjeta Flotante (Arriba izquierda) -->
  <div class="absolute -right-20 top-12 bg-shiori-card/90 backdrop-blur border border-shiori-cyan rounded-xl p-3 shadow-xl z-20 flex items-center gap-3">
    <div class="w-8 h-8 rounded-lg bg-green-500/20 flex items-center justify-center">
        <svg class="w-4 h-4 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
    </div>
    <div>
      <div class="text-sm font-bold text-white">Auditoría OK</div>
      <div class="text-[10px] text-gray-400">Hace 2 min</div>
    </div>
  </div>

  <!-- Ventana Principal del Mockup -->
  <div class="bg-[#0b162c] border border-shiori-card rounded-2xl shadow-2xl overflow-hidden transform transition-transform hover:-translate-y-2 duration-500 relative z-10">
    
    <!-- Topbar de la ventana -->
    <div class="bg-shiori-dark/80 px-4 py-3 border-b border-shiori-card flex items-center justify-between">
      <div class="flex gap-1.5">
        <div class="w-2.5 h-2.5 rounded-full bg-red-500/80"></div>
        <div class="w-2.5 h-2.5 rounded-full bg-yellow-500/80"></div>
        <div class="w-2.5 h-2.5 rounded-full bg-green-500/80"></div>
      </div>
      <div class="text-[10px] text-gray-400 font-medium tracking-widest uppercase">Shiori Dashboard</div>
      <div class="flex items-center gap-1.5 text-[9px] text-shiori-turq font-medium">
        <span class="w-1.5 h-1.5 rounded-full bg-shiori-turq animate-pulse"></span> LIVE
      </div>
    </div>

    <!-- Cuerpo del Dashboard -->
    <div class="p-5">
      <!-- KPIs -->
      <div class="grid grid-cols-3 gap-3 mb-5">
        <div class="bg-shiori-card/40 border border-shiori-card/60 rounded-lg p-3">
          <div class="text-[9px] text-gray-400 uppercase tracking-wider mb-1">Ingresos</div>
          <div class="font-display font-bold text-lg text-white">$84,291</div>
          <div class="text-[10px] text-shiori-turq mt-0.5">+2.4% hoy</div>
        </div>
        <div class="bg-shiori-card/40 border border-shiori-card/60 rounded-lg p-3">
          <div class="text-[9px] text-gray-400 uppercase tracking-wider mb-1">Egresos</div>
          <div class="font-display font-bold text-lg text-white">$12,840</div>
          <div class="text-[10px] text-gray-500 mt-0.5">Mes actual</div>
        </div>
        <div class="bg-shiori-card/40 border border-shiori-card/60 rounded-lg p-3">
          <div class="text-[9px] text-gray-400 uppercase tracking-wider mb-1">Anomalías</div>
          <div class="font-display font-bold text-lg text-white">0</div>
          <div class="text-[10px] text-shiori-turq mt-0.5">Seguro</div>
        </div>
      </div>

      <!-- Gráfico Falso -->
      <div class="bg-shiori-card/40 border border-shiori-card/60 rounded-lg p-4 mb-4">
        <div class="flex justify-between items-center mb-4">
          <div class="text-xs text-gray-300 font-medium">Volumen — 7 días</div>
          <div class="text-[9px] bg-shiori-turq/10 text-shiori-turq border border-shiori-turq/20 px-2 py-0.5 rounded-full">+12.4%</div>
        </div>
        <div class="flex items-end gap-1.5 h-14">
          <div class="flex-1 bg-shiori-light/20 border-t-2 border-shiori-light rounded-t-sm h-[40%]"></div>
          <div class="flex-1 bg-shiori-light/20 border-t-2 border-shiori-light rounded-t-sm h-[65%]"></div>
          <div class="flex-1 bg-shiori-light/20 border-t-2 border-shiori-light rounded-t-sm h-[45%]"></div>
          <div class="flex-1 bg-shiori-light/20 border-t-2 border-shiori-light rounded-t-sm h-[85%]"></div>
          <div class="flex-1 bg-shiori-light/20 border-t-2 border-shiori-light rounded-t-sm h-[55%]"></div>
          <div class="flex-1 bg-shiori-turq/30 border-t-2 border-shiori-turq rounded-t-sm h-[95%]"></div>
        </div>
      </div>

      <!-- Lista de transacciones -->
      <div class="space-y-2">
        <div class="flex items-center gap-3 bg-shiori-card/20 border border-shiori-card/50 rounded-lg p-2.5">
          <div class="w-7 h-7 rounded bg-shiori-dark text-shiori-turq flex items-center justify-center text-[9px] font-bold">CX</div>
          <div class="flex-1">
            <div class="text-xs text-gray-200 font-medium">Cierre Contable</div>
            <div class="text-[9px] text-gray-500">Hoy, 9:14 AM</div>
          </div>
          <div class="text-xs text-shiori-turq font-bold">Aprobado</div>
        </div>
      </div>
    </div>
  </div>
</div>
