import React from 'react';
import { secretObject } from '../../pages/api/models/secret.object';

interface ModalProps {
  onClose: () => void;
}

const EmptySecretsOrNotLoggedInModal: React.FC<ModalProps> = ({ onClose }) => {
  const handleAction = () => {
    onClose();
  };

  return (
    <div className="modal">
      <div className="modal-content">
        <div className='modal-input-container'>
        <h2>You need to log in</h2>
        <p>Please log in to see your stored passwords.</p>
        </div>
        <div className='modal-buttons-container'>
        <button onClick={onClose}>Log In</button>
        </div>
      </div>
    </div>
  );
};

export default EmptySecretsOrNotLoggedInModal;