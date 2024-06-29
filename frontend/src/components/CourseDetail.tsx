import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useParams } from 'react-router-dom';

interface Course {
  id: number;
  title: string;
  description: string;
  duration: number;
  category: string;
}

const CourseDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [course, setCourse] = useState<Course | null>(null);

  useEffect(() => {
    axios.get<Course>(`http://localhost:8080/courses/${id}`)
      .then(response => {
        setCourse(response.data);
      })
      .catch(error => {
        console.error('There was an error fetching the course!', error);
      });
  }, [id]);

  if (!course) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <h2>{course.title}</h2>
      <p>{course.description}</p>
      <p>Duration: {course.duration} minutes</p>
      <p>Category: {course.category}</p>
    </div>
  );
};

export default CourseDetail;
