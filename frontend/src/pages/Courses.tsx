
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';
import CourseList from '../components/CourseList';
import { Link } from 'react-router-dom';

interface Course {
  id: number;
  title: string;
  description: string;
  duration: number;
  category: string;
}

const Courses: React.FC = () => {
  const [courses, setCourses] = useState<Course[]>([]);
  const [error, setError] = useState('');
  const { user } = useAuth();

  useEffect(() => {
    const fetchCourses = async () => {
      try {
        const token = localStorage.getItem('token');
        const response = await axios.get('http://localhost:8080/courses', {
          headers: { Authorization: `Bearer ${token}` }
        });
        setCourses(response.data);
      } catch (err) {
        setError('There was an error fetching the courses!');
      }
    };

    fetchCourses();
  }, []);

  return (
    <div>
      <h1>Courses</h1>
      {error && <p>{error}</p>}
      <CourseList courses={courses} />
      {user && (
        <div>
          <Link to="/create-course">Create New Course</Link>
        </div>
      )}
    </div>
  );
};

export default Courses;

