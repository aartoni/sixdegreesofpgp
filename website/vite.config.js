import react from '@vitejs/plugin-react';
import {defineConfig, loadEnv} from 'vite';
import {VitePWA} from 'vite-plugin-pwa';

export default defineConfig(({mode}) => {
  // eslint-disable-next-line no-undef
  const env = loadEnv(mode, process.cwd());

  return {
    server: {
      port: 3000,
      manifest: true,
      open: true, // Open browser when server starts.
    },
    plugins: [
      react(),
      VitePWA({
        includeAssets: ['apple-touch-icon-180x180.png'],
        manifest: {
          name: `Six Degrees of ${env.VITE_TOPIC}`,
          short_name: `SDO${env.VITE_TOPIC[0]}`,
          description: env.VITE_DESCRIPTION,
          display: 'standalone',
          background_color: '#68ceff',
          theme_color: '#68ceff',
          icons: [
            {
              src: 'favicon-16x16.png',
              type: 'image/png',
              sizes: '16x16',
            },
            {
              src: 'favicon-32x32.png',
              type: 'image/png',
              sizes: '32x32',
            },
            {
              src: 'favicon-96x96.png',
              type: 'image/png',
              sizes: '96x96',
            },
            {
              src: 'favicon-192x192.png',
              sizes: '192x192',
              type: 'image/png',
            },
          ],
        },
      }),
    ],
  };
});
