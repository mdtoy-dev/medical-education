

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import axios from 'axios';

interface AuthContextType {
  user: { username?: string } | null;
  login: (token: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<{ username?: string } | null>(null);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      axios.get('http://localhost:8080/auth/profile', {
        headers: { Authorization: `Bearer ${token}` }
      })
        .then(response => {
          setUser({ username: response.data.username });
        })
        .catch(error => console.error('Error fetching profile', error));
    }
  }, []);

  const login = (token: string) => {
    localStorage.setItem('token', token);
    axios.get('http://localhost:8080/auth/profile', {
      headers: { Authorization: `Bearer ${token}` }
    })
      .then(response => {
        setUser({ username: response.data.username });
      })
      .catch(error => console.error('Error fetching profile', error));
  };

  const logout = () => {
    localStorage.removeItem('token');
    setUser(null);
    window.location.href = '/login';
  };

  return (
    <AuthContext.Provider value={{ user, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

