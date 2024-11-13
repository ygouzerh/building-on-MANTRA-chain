import React from 'react';
import ReactDOM from 'react-dom/client';
import { ChakraProvider } from '@chakra-ui/react';
import { GrazProvider } from 'graz';
import App from './App';
import { mantraChainConfig } from './chain';

const grazConfig = {
  chains: [mantraChainConfig],
  defaultChain: mantraChainConfig,
};

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <GrazProvider grazOptions={grazConfig}>
      <ChakraProvider>
        <App />
      </ChakraProvider>
    </GrazProvider>
  </React.StrictMode>
);