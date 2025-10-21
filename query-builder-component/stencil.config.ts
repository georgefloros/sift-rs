import { Config } from '@stencil/core';

export const config: Config = {
  namespace: 'query-builder-component',
  outputTargets: [
    {
      type: 'dist',
      esmLoaderPath: '../loader',
    },
    {
      type: 'dist-custom-elements',
      customElementsExportBehavior: 'auto-define-custom-elements',
      externalRuntime: false,
    },
    {
      type: 'docs-readme',
    },
    {
      type: 'www',
      serviceWorker: null, // disable service workers
      copy: [
        // Copy WASM files to www directory for serving
        {
          src: '../../sift-rs-wasm/pkg',
          dest: 'sift-rs-wasm/pkg',
          warn: false
        }
      ]
    },
  ],
  testing: {
    browserHeadless: "shell",
  },
  devServer: {
    reloadStrategy: 'hmr',
    port: 3333,
  }
};
