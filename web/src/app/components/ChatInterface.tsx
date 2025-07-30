'use client';

import React, { useState, useEffect, useRef } from 'react';
import { Send, CornerDownLeft, MessageSquare, X } from 'lucide-react';

interface ChatMessage {
  id: string;
  sender: 'user' | 'ai';
  text: string;
  query?: string;
  explanation?: string;
}

interface ChatInterfaceProps {
  onQueryUpdate: (query: string) => void;
  jsonInput: string;
}

export const ChatInterface: React.FC<ChatInterfaceProps> = ({ onQueryUpdate, jsonInput }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const ws = useRef<WebSocket | null>(null);
  const messageEndRef = useRef<HTMLDivElement | null>(null);

  const connectWebSocket = () => {
    const socket = new WebSocket(process.env.NEXT_PUBLIC_CHAT_API_URL || 'ws://localhost:3001/ws');

    socket.onopen = () => {
      console.log('WebSocket connected');
      // Add welcome message
      setMessages([{
        id: 'welcome',
        sender: 'ai',
        text: 'Hi! I can help you build MongoDB queries in natural language. Just tell me what you want to find!',
      }]);
    };
    
    socket.onclose = () => {
      console.log('WebSocket disconnected');
      // Try to reconnect after a delay
      setTimeout(() => {
        if (isOpen) connectWebSocket();
      }, 3000);
    };
    
    socket.onerror = (error) => {
      console.error('WebSocket error:', error);
      setMessages(prev => [...prev, {
        id: `error-${Date.now()}`,
        sender: 'ai',
        text: 'Connection error. Please check if the chat backend is running.',
      }]);
    };

    socket.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);

        if (message.type === 'ai_response') {
          setMessages((prev) => [
            ...prev,
            {
              id: message.id,
              sender: 'ai',
              text: message.message,
              query: message.query,
              explanation: message.explanation,
            },
          ]);

          if (message.query) {
            try {
              // Validate that the query is valid JSON before updating
              JSON.parse(message.query);
              onQueryUpdate(message.query);
            } catch (e) {
              console.error('Invalid query JSON from AI:', e);
            }
          }
        } else if (message.type === 'error') {
          setMessages((prev) => [
            ...prev,
            {
              id: message.id,
              sender: 'ai',
              text: `Error: ${message.error}`,
            },
          ]);
        }
      } catch (e) {
        console.error('Failed to parse WebSocket message:', e);
      }
    };

    ws.current = socket;
  };

  useEffect(() => {
    if (isOpen) {
      connectWebSocket();
    }

    return () => {
      ws.current?.close();
    };
  }, [isOpen, onQueryUpdate]);

  useEffect(() => {
    messageEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSendMessage = () => {
    if (inputValue.trim() && ws.current?.readyState === WebSocket.OPEN) {
      const userMessage: ChatMessage = {
        id: `user-${Date.now()}`,
        sender: 'user',
        text: inputValue,
      };

      setMessages((prev) => [...prev, userMessage]);

      try {
        const parsedJson = JSON.parse(jsonInput);
        const messageToSend = {
          type: 'user_message',
          id: userMessage.id,
          message: inputValue,
          sample_data: parsedJson,
        };

        ws.current.send(JSON.stringify(messageToSend));
      } catch (error) {
        console.error("Invalid JSON input:", error);
        // Handle invalid JSON input gracefully
      }

      setInputValue('');
    }
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 bg-blue-600 text-white p-4 rounded-full shadow-lg hover:bg-blue-700 transition-colors"
      >
        <MessageSquare className="w-8 h-8" />
      </button>
    );
  }

  return (
    <div className="fixed bottom-4 right-4 w-96 h-[600px] bg-white rounded-lg shadow-xl border flex flex-col">
      <div className="flex justify-between items-center p-4 border-b bg-gray-50">
        <h2 className="text-lg font-semibold">AI Query Assistant</h2>
        <button onClick={() => setIsOpen(false)} className="text-gray-500 hover:text-gray-700">
          <X className="w-6 h-6" />
        </button>
      </div>
      <div className="flex-1 p-4 overflow-y-auto">
        <div className="space-y-4">
          {messages.map((msg) => (
            <div key={msg.id} className={`flex ${msg.sender === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`p-3 rounded-lg max-w-xs ${msg.sender === 'user' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-800'}`}>
                <p>{msg.text}</p>
                {msg.query && (
                  <div className="mt-2 p-2 bg-gray-800 text-white rounded-md text-sm">
                    <pre><code>{JSON.stringify(JSON.parse(msg.query), null, 2)}</code></pre>
                  </div>
                )}
                {msg.explanation && <p className="text-xs mt-2">{msg.explanation}</p>}
              </div>
            </div>
          ))}
          <div ref={messageEndRef} />
        </div>
      </div>
      <div className="p-4 border-t">
        <div className="relative">
          <input
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
            placeholder="Ask me to build a query..."
            className="w-full pr-12 pl-4 py-3 border rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={handleSendMessage}
            className="absolute right-2 top-1/2 -translate-y-1/2 bg-blue-600 text-white p-2.5 rounded-full hover:bg-blue-700 transition-colors"
          >
            <Send className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  );
};

