
import React, { useEffect, useState } from 'react';

const initWasm = async () => {
  const wasm = await import('../../../backend/wasm_module/pkg/wasm_module');
  return wasm;
};



const ContentCreator: React.FC = () => {
  const [wasm, setWasm] = useState<any>(null);
  const [title, setTitle] = useState('');
  const [body, setBody] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [author, setAuthor] = useState('');
  const [content, setContent] = useState<any>(null);

  useEffect(() => {
    initWasm().then(setWasm);
  }, []);

  const handleCreate = () => {
    const newContent = wasm.create_content(title, body, author, JSON.stringify(tags));
    setContent(newContent);
  };

  const handleUpdate = () => {
    const updatedContent = wasm.update_content(content, title, body, JSON.stringify(tags));
    setContent(updatedContent);
  };

  const handleTagsChange = (index: number, value: string) => {
    const newTags = [...tags];
    newTags[index] = value;
    setTags(newTags);
  };

  const addTag = () => {
    setTags([...tags, '']);
  };

  return (
    <div>
      <h1>Content Creator</h1>
      <input
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="Title"
      />
      <textarea
        value={body}
        onChange={(e) => setBody(e.target.value)}
        placeholder="Body"
      ></textarea>
      <input
        type="text"
        value={author}
        onChange={(e) => setAuthor(e.target.value)}
        placeholder="Author"
      />
      <div>
        <label>Tags:</label>
        {tags.map((tag, index) => (
          <input
            key={index}
            type="text"
            value={tag}
            onChange={(e) => handleTagsChange(index, e.target.value)}
          />
        ))}
        <button onClick={addTag}>Add Tag</button>
      </div>
      <button onClick={handleCreate}>Create Content</button>
      <button onClick={handleUpdate} disabled={!content}>
        Update Content
      </button>
      {content && (
        <div>
          <h2>{content.title}</h2>
          <p>{content.body}</p>
          <p>Author: {content.metadata.author}</p>
          <p>Tags: {content.tags.join(', ')}</p>
        </div>
      )}
    </div>
  );
};

export default ContentCreator;

