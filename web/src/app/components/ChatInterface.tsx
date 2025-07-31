'use client';

import React, { useState, useEffect, useRef } from 'react';
import { Send, CornerDownLeft, MessageSquare, X, Check, GitMerge, RefreshCw } from 'lucide-react';

interface ChatMessage {
  id: string;
  sender: 'user' | 'ai';
  text: string;
  query?: string;
  explanation?: string;
  queryAccepted?: boolean;
}

interface ChatInterfaceProps {
  onQueryUpdate: (query: string) => void;
  jsonInput: string;
  currentQuery?: string;
}

export const ChatInterface: React.FC<ChatInterfaceProps> = ({ onQueryUpdate, jsonInput, currentQuery = '{}' }) => {
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
              queryAccepted: false,
            },
          ]);

          // Remove automatic query update - user must now accept it manually
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
    console.log('ChatInterface useEffect triggered, isOpen:', isOpen);
    if (isOpen) {
      connectWebSocket();
    }

    return () => {
      ws.current?.close();
    };
  }, [isOpen]); // Remove onQueryUpdate from dependencies to prevent circular updates

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

        // Build conversation history for context
        const conversationHistory = messages
          .filter(msg => msg.sender === 'user' || (msg.sender === 'ai' && msg.text))
          .slice(-100) // Keep last 100 messages for context
          .map(msg => ({
            role: msg.sender === 'user' ? 'user' : 'assistant',
            content: msg.text
          }));

        const messageToSend = {
          type: 'user_message',
          id: userMessage.id,
          message: inputValue,
          sample_data: parsedJson,
          current_query: currentQuery,
          conversation_history: conversationHistory,
        };

        ws.current.send(JSON.stringify(messageToSend));
      } catch (error) {
        console.error("Invalid JSON input:", error);
        // Handle invalid JSON input gracefully
      }

      setInputValue('');
    }
  };

  const handleReplaceQuery = (messageId: string, query: string) => {
    try {
      // Validate that the query is valid JSON before updating
      JSON.parse(query);
      console.log('Replace query called with:', query);
      onQueryUpdate(query);

      // Mark the query as accepted in the message
      setMessages(prev =>
        prev.map(msg =>
          msg.id === messageId
            ? { ...msg, queryAccepted: true }
            : msg
        )
      );
    } catch (e) {
      console.error('Invalid query JSON from AI:', e);
    }
  };

  const handleMergeQuery = (messageId: string, aiQuery: string) => {
    try {
      // Validate that both queries are valid JSON
      const parsedAiQuery = JSON.parse(aiQuery);
      const parsedCurrentQuery = JSON.parse(currentQuery);

      // Merge the queries using MongoDB $and operator if current query is not empty
      let mergedQuery;
      if (Object.keys(parsedCurrentQuery).length === 0) {
        // If current query is empty, just use the AI query
        mergedQuery = parsedAiQuery;
      } else {
        // Merge both queries with $and
        mergedQuery = Object.assign(parsedAiQuery, parsedCurrentQuery);
      }

      console.log('Merge query called, merged result:', mergedQuery);
      onQueryUpdate(JSON.stringify(mergedQuery, null, 2));

      // Mark the query as accepted in the message
      setMessages(prev =>
        prev.map(msg =>
          msg.id === messageId
            ? { ...msg, queryAccepted: true }
            : msg
        )
      );
    } catch (e) {
      console.error('Invalid query JSON for merging:', e);
    }
  };

  const isValidQuery = (query: string): boolean => {
    try {
      JSON.parse(query);
      return true;
    } catch (e) {
      return false;
    }
  };

  if (!isOpen) {
    return (
      <button title='Open AI Query Assistant'
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
        <button title="Close AI Query Assistant" onClick={() => setIsOpen(false)} className="text-gray-500 hover:text-gray-700">
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

                {/* Action Buttons - Only show for AI messages with valid queries that haven't been accepted */}
                {msg.sender === 'ai' && msg.query && isValidQuery(msg.query) && !msg.queryAccepted && (
                  <div className="mt-3 flex justify-center space-x-2">
                    <button
                      onClick={() => handleMergeQuery(msg.id, msg.query!)}
                      className="flex items-center space-x-1 px-2 py-1.5 bg-blue-600 text-white text-xs rounded-md hover:bg-blue-700 transition-colors"
                      title="Merge with existing query using $and operator"
                    >
                      <GitMerge className="w-3 h-3" />
                      <span>Merge</span>
                    </button>
                    <button
                      onClick={() => handleReplaceQuery(msg.id, msg.query!)}
                      className="flex items-center space-x-1 px-2 py-1.5 bg-green-600 text-white text-xs rounded-md hover:bg-green-700 transition-colors"
                      title="Replace current query completely"
                    >
                      <RefreshCw className="w-3 h-3" />
                      <span>Replace</span>
                    </button>
                  </div>
                )}

                {/* Show acceptance confirmation */}
                {msg.sender === 'ai' && msg.query && msg.queryAccepted && (
                  <div className="mt-2 flex items-center space-x-1 text-green-600 text-xs">
                    <Check className="w-3 h-3" />
                    <span>Query applied successfully</span>
                  </div>
                )}
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
          <button title='Send message'
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

