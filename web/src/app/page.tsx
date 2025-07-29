'use client';

import { SiftValidator } from './components/SiftValidator';

export default function Home() {
  return (
    <div className="min-h-screen bg-gray-50">
      <div className="container mx-auto px-4 py-8">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">
            Sift-rs MongoDB Query Validator
          </h1>
          <p className="text-gray-600 max-w-2xl mx-auto">
            Build and test MongoDB queries against JSON data in real-time using the sift-rs validation engine.
          </p>
        </div>
        <SiftValidator />
      </div>
    </div>
  );
}
