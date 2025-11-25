import React from 'react';
import ReactDOM from 'react-dom/client';
import { QueryProvider } from '@/app/provider/query-client';
import { HomePage } from '@/pages/home/HomePage';
import '@/app/styles/tailwind.css';

const root = document.getElementById('root');

if (!root) {
  throw new Error('Root element not found');
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <QueryProvider>
      <HomePage />
    </QueryProvider>
  </React.StrictMode>
);
