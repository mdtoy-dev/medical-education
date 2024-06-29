import React from 'react';
import { Link } from 'react-router-dom';

const Home: React.FC = () => {
  return (
    <div>
      <h1>Welcome to Medical Education Platform</h1>
      <Link to="/courses">View Courses</Link>
      <br />
      <Link to="/register">Register</Link>
      <br />
      <Link to="/login">Login</Link>
    </div>
  );
};

export default Home;

