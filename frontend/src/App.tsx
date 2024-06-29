import React from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate, Link } from 'react-router-dom';
import Home from './pages/Home';
import Courses from './pages/Courses';
import CourseDetailPage from './pages/CourseDetailPage';
import RegisterForm from './components/RegisterForm';
import LoginForm from './components/LoginForm';
import Profile from './pages/Profile';
import CourseForm from './components/CourseForm';
import { AuthProvider, useAuth } from './context/AuthContext';
import ContentCreator from './components/ContentCreator';

const PrivateRoute: React.FC<{ children: JSX.Element }> = ({ children }) => {
  const { user } = useAuth();
  return user ? children : <Navigate to="/login" />;
};

const AdminRoute: React.FC<{ children: JSX.Element }> = ({ children }) => {
  const { user } = useAuth();
  return user?.username === 'admin' ? children : <Navigate to="/" />;
};

const App: React.FC = () => {
  const { user, logout } = useAuth();

  return (
    <Router>
      <nav>
        <Link to="/">Home</Link> |
        <Link to="/courses">Courses</Link> |
        {!user && <Link to="/register">Register</Link>} |
        {!user && <Link to="/login">Login</Link>} |
        {user && <Link to="/profile">Profile</Link>} |
        {user?.username === 'admin' && <Link to="/create-course">Create Course</Link>} |
        {user && <button onClick={logout}>Logout</button>}
      </nav>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/courses" element={<PrivateRoute><Courses /></PrivateRoute>} />
        <Route path="/courses/:id" element={<PrivateRoute><CourseDetailPage /></PrivateRoute>} />
        <Route path="/register" element={<RegisterForm />} />
        <Route path="/login" element={<LoginForm />} />
        <Route path="/profile" element={<PrivateRoute><Profile /></PrivateRoute>} />
        <Route path="/create-course" element={<AdminRoute><CourseForm onCourseSaved={() => window.location.href = '/courses'} /></AdminRoute>} />
        <Route path="/edit-course/:id" element={<AdminRoute><CourseForm onCourseSaved={() => window.location.href = '/courses'} /></AdminRoute>} />
      </Routes>
    </Router>
  );
};

const WrappedApp: React.FC = () => (
  <AuthProvider>
    <App />
    <ContentCreator />
  </AuthProvider>
);

export default WrappedApp;

