export const API_URL = import.meta.env.VITE_API_URL;
export const THIRD_PARTY_API_URL = import.meta.env.VITE_THIRD_PARTY_API_URL;

const AUTHOR_EMAIL = import.meta.env.VITE_AUTHOR_EMAIL;
const URL = import.meta.env.VITE_PUBLIC_URL;
export const TOPIC = import.meta.env.VITE_APP_TOPIC;
export const USER_AGENT = `Six Degrees of ${TOPIC}/1.0 (${URL}; ${AUTHOR_EMAIL})`;
