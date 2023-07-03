import React from 'react';
import { secretObject } from '../../pages/api/models/secret.object';
import { on } from 'events';
import { create_secret } from '../../pages/api/secret/secret';
import { useRouter } from 'next/router';

interface ModalProps {
  onClose: () => void;
  onAdd: () => void;
}

const AddSecretModal: React.FC<ModalProps> = ({ onClose, onAdd } : ModalProps) => {

  const [input, setInput] = React.useState({
    secret_title: "",
    secret_content: ""
});

const handleInputChange = (event: { target: { name: any; value: any; }; }) => {
  const { name, value } = event.target;
  setInput((prevState) => ({
    ...prevState,
    [name]: value
  }));
};

  const router = useRouter();
  
  const onAction = async (secret_title: string, secret_content: string) => {
    let token : string | null = localStorage.getItem('token');
    if (token !== null) {
      await create_secret(secret_title, secret_content, token);
      //router.push('/secrets');
      onAdd();
    }
  }

  const handleClick = () => {
    onAction(input.secret_title, input.secret_content);
    onClose();
  }


  return (
    <div className="modal">
      <div className="modal-content">
        <h2>New Secret</h2>
        <div className='modal-input-container'>
        <input className='modal-input' name='secret_title' placeholder='Name' value={input.secret_title} onChange={handleInputChange}/><br/>
        <input className='modal-input' name='secret_content' placeholder='Password' value={input.secret_content} onChange={handleInputChange} />
        </div>
        <div className='modal-buttons-container'>
          {
              <button className='submit' onClick={handleClick}>Perform Action</button>
          }
        <button onClick={onClose}>Close</button>
        </div>
      </div>
    </div>
  );
};

export default AddSecretModal;