
import React from 'react';
import { Link } from 'react-router-dom';

interface Course {
  id: number;
  title: string;
  description: string;
  duration: number;
  category: string;
}

interface Props {
  courses: Course[];
}

const CourseList: React.FC<Props> = ({ courses }) => {
  return (
    <div>
      <h2>Courses</h2>
      <ul>
        {courses.map(course => (
          <li key={course.id}>
            <Link to={`/courses/${course.id}`}>{course.title}</Link>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default CourseList;

