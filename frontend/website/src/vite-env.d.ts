/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_URL: string;
  readonly VITE_AUTHOR_EMAIL: string;
  readonly VITE_AUTHOR_NAME: string;
  readonly VITE_DESCRIPTION: string;
  readonly VITE_THIRD_PARTY_API_URL: string;
  readonly VITE_TOPIC: string;
  readonly VITE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
