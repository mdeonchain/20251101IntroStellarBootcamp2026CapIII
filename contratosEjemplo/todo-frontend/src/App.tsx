function App() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-950 text-gray-100">
      <h1 className="text-4xl font-bold mb-4 text-blue-400">todo-frontend</h1>
      <p className="text-lg text-gray-400">Base React + Vite + TypeScript + TailwindCSS</p>
      <button className="mt-6 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors">
        ¡Todo listo para conectar con Rust 🦀!
      </button>
    </div>
  );
}

export default App;
