
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';

const Profile: React.FC = () => {
  const { user, logout } = useAuth();
  const [profile, setProfile] = useState({ username: '', email: '' });
  const [newEmail, setNewEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    setLoading(true);
    const token = localStorage.getItem('token');
    axios.get('http://localhost:8080/auth/profile', {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
      .then(response => {
        setProfile(response.data);
        setLoading(false);
      })
      .catch(error => {
        console.error('There was an error fetching the profile!', error);
        setError('Failed to fetch profile.');
        setLoading(false);
      });
  }, []);

  const handleUpdateProfile = () => {
    setLoading(true);
    const token = localStorage.getItem('token');
    axios.put('http://localhost:8080/auth/profile', { email: newEmail }, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
      .then(response => {
        alert('Profile updated successfully');
        setProfile(response.data);
        setLoading(false);
      })
      .catch(error => {
        console.error('There was an error updating the profile!', error);
        setError('Failed to update profile.');
        setLoading(false);
      });
  };

  return (
    <div>
      <h1>Profile</h1>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <>
          <p>Username: {profile.username}</p>
          <p>Email: {profile.email}</p>
          <div>
            <label>New Email:</label>
            <input type="email" value={newEmail} onChange={(e) => setNewEmail(e.target.value)} />
            <button onClick={handleUpdateProfile}>Update Profile</button>
            <button onClick={logout}>Logout</button>
          </div>
          {error && <p>{error}</p>}
        </>
      )}
    </div>
  );
};

export default Profile;

