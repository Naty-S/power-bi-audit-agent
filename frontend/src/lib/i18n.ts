import { writable, derived } from 'svelte/store';

export const locale = writable('es');

const translations = {
  es: {
    nav: { home: 'Inicio', features: 'Características', pricing: 'Planes', team: 'Especialistas', contact: 'Contacto', demo: 'Ir al Demo' },
    hero: {
      title: 'SHIORI: El guardián inteligente de la integridad financiera.',
      subtitle: 'Shiori (Japonés): El marcador. La guía que ilumina los datos corporativos.',
      obj: 'Automatización del proceso de evaluación de operaciones financieras y controles de seguridad de la información en el ERP.'
    },
    benefits: [
      {
        title: 'Seguridad de la información',
        text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
        tags: ["SOC 2 Type II", "Encriptación AES", "Monitoreo 24/7"],
        icon: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"
      },
      {
        title: 'Cumplimiento de la normativa técnica',
        text: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.',
        tags: ['ISO 27001', "Auditoría continua", "Reportes legales"],
        icon: "M3 9h18M9 21V9"
      },
      {
        title: 'Visualización digital de la salud financiera y operativa del negocio',
        text: 'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nullam do magna sit amet dictum.',
        tags: ["Tiempo real", "Métricas clave", 'Dashboards interactivos'],
        icon: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"
      }
    ],
    features: { title: 'Nuestros Pilares', f1: 'Integración de datos', f2: 'Análisis de Big Data', f3: 'Detección en tiempo real', f4: 'Presentación de resultados' },
    pricing: { title: 'Planes', basic: 'Plan Básico', pro: 'Plan Crecimiento', payg: 'Pago por Uso' },
    team: { title: 'Especialistas' },
    contact: { title: 'Contacto' }
  },
  en: {
    nav: { home: 'Home', features: 'Features', pricing: 'Pricing', team: 'Specialists', contact: 'Contact', demo: 'Go to Demo' },
    hero: {
      title: 'SHIORI: The intelligent guardian of financial integrity.',
      subtitle: 'Shiori (Japanese): The bookmark. The guide that illuminates corporate data.',
      obj: 'Automation of the financial operations evaluation process and information security controls in the ERP.'
    },
    benefits: [
      {
        title: 'Seguridad de la información',
        text: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
        tags: ["SOC 2 Type II", "Encriptación AES", "Monitoreo 24/7"] },
      {
        title: 'Cumplimiento de la normativa técnica',
        text: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.',
        tags: ['ISO 27001', "Auditoría continua", "Reportes legales"] },
      {
        title: 'Visualización digital de la salud financiera y operativa del negocio',
        text: 'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nullam do magna sit amet dictum.',
        tags: ["Tiempo real","Métricas clave",'Dashboards interactivos'] }
    ],
    features: { title: 'Our Pillars', f1: 'Data Integration', f2: 'Big Data Analysis', f3: 'Real-time detection', f4: 'Results Presentation' },
    pricing: { title: 'Plans', basic: 'Basic Plan', pro: 'Growth Plan', payg: 'Pay As You Go' },
    team: { title: 'Specialists' },
    contact: { title: 'Contact' }
  }
};

export const t = derived(locale, ($locale: string) => (key: string) => {

  const keys = key.split('.');
  let value = translations[$locale];
  

  for (const k of keys) {
    if (value === undefined) return key;
    value = value[k];
  }

  return value || key;
});
