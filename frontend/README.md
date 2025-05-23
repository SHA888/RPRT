# RPRT Frontend

This is the frontend for the RPRT (Rust PostgreSQL React TypeScript) application. It's built with:

- [React](https://reactjs.org/) - A JavaScript library for building user interfaces
- [TypeScript](https://www.typescriptlang.org/) - TypeScript is a typed superset of JavaScript
- [Vite](https://vitejs.dev/) - Next Generation Frontend Tooling
- [Mantine](https://mantine.dev/) - A fully featured React components library
- [React Query](https://tanstack.com/query/latest) - Data-fetching and state management
- [Axios](https://axios-http.com/) - Promise based HTTP client for the browser and node.js

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## Getting Started

### Prerequisites

- Node.js (v16 or later)
- npm or yarn

### Installation

1. Install dependencies:

```bash
npm install
# or
yarn install
```

2. Create a `.env` file in the root directory and add the following environment variables:

```env
VITE_API_BASE_URL=http://localhost:3000/api
```

### Available Scripts

- `npm run dev` - Start the development server
- `npm run build` - Build the application for production
- `npm run preview` - Preview the production build
- `npm run lint` - Run ESLint
- `npm run type-check` - Run TypeScript type checking

## Project Structure

```
src/
├── api/               # API client and services
├── assets/            # Static assets
├── components/        # Reusable components
├── features/          # Feature-based modules
├── hooks/             # Custom React hooks
├── providers/         # Context providers
├── styles/            # Global styles
├── types/             # TypeScript type definitions
├── utils/             # Utility functions
├── App.tsx            # Root component
└── main.tsx           # Application entry point
```

## Development

To start the development server:

```bash
npm run dev
```

The application will be available at `http://localhost:5173`

## Building for Production

To create a production build:

```bash
npm run build
```

The build artifacts will be stored in the `dist/` directory.
  },
})
```
