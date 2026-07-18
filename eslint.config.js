import tsPlugin from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import jsxA11yPlugin from 'eslint-plugin-jsx-a11y';
import reactPlugin from 'eslint-plugin-react';
import reactHooksPlugin from 'eslint-plugin-react-hooks';
import simpleImportSort from 'eslint-plugin-simple-import-sort';

export default [
  {
    ignores: ['node_modules/', 'dist/', 'env.d.ts', 'vite-env.d.ts'],
  },
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true,
        },
      },
      globals: {
        console: 'readonly',
        document: 'readonly',
        HTMLElement: 'readonly',
        navigator: 'readonly',
        window: 'readonly',
      },
    },
    plugins: {
      '@typescript-eslint': tsPlugin,
      react: reactPlugin,
      'react-hooks': reactHooksPlugin,
      'jsx-a11y': jsxA11yPlugin,
      'simple-import-sort': simpleImportSort,
    },
    settings: {
      react: {
        version: 'detect',
      },

    },
    rules: {
      'simple-import-sort/imports': [
        'error',
        {
          groups: [
            ['^node:'],
            ['^react$', '^react-dom$'],
            ['^@?\\w'],
            ['^core(/|$)'],
            ['^shared(/|$)'],
            ['^features(/|$)'],
            ['^pages(/|$)'],
            ['^assets(/|$)'],
            ['^\\.'],
            ['^\\u0000'],
          ],
        },
      ],
      'simple-import-sort/exports': 'error',
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          vars: 'all',
          args: 'after-used',
          ignoreRestSiblings: true,
          argsIgnorePattern: '^_',
        },
      ],
      'react/react-in-jsx-scope': 'off',
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
    },
  },
];
