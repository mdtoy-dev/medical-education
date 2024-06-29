
import React, { useState, useEffect } from 'react';
import axios from 'axios';

interface CourseFormProps {
  courseId?: number;
  onCourseSaved: () => void;
}

const CourseForm: React.FC<CourseFormProps> = ({ courseId, onCourseSaved }) => {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [duration, setDuration] = useState<number | null>(null);
  const [category, setCategory] = useState('');

  useEffect(() => {
    const fetchCourse = async () => {
      if (courseId) {
        const token = localStorage.getItem('token');
        const response = await axios.get(`http://localhost:8080/courses/${courseId}`, {
          headers: { Authorization: `Bearer ${token}` }
        });
        const { title, description, duration, category } = response.data;
        setTitle(title);
        setDescription(description);
        setDuration(duration);
        setCategory(category);
      }
    };

    fetchCourse();
  }, [courseId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const token = localStorage.getItem('token');
    const data = { title, description, duration, category };

    try {
      if (courseId) {
        await axios.put(`http://localhost:8080/courses/${courseId}`, data, {
          headers: { Authorization: `Bearer ${token}` }
        });
      } else {
        await axios.post('http://localhost:8080/courses', data, {
          headers: { Authorization: `Bearer ${token}` }
        });
      }
      onCourseSaved();
    } catch (err) {
      console.error('There was an error saving the course!', err);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <div>
        <label>Title</label>
        <input type="text" value={title} onChange={(e) => setTitle(e.target.value)} required />
      </div>
      <div>
        <label>Description</label>
        <textarea value={description} onChange={(e) => setDescription(e.target.value)} required></textarea>
      </div>
      <div>
        <label>Duration (in minutes)</label>
        <input type="number" value={duration ?? ''} onChange={(e) => setDuration(e.target.value ? parseInt(e.target.value) : null)} />
      </div>
      <div>
        <label>Category</label>
        <input type="text" value={category} onChange={(e) => setCategory(e.target.value)} />
      </div>
      <button type="submit">Save Course</button>
    </form>
  );
};

export default CourseForm;

