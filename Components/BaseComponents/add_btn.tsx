import React, { useState } from 'react';
import { secretObject } from '../../pages/api/models/secret.object';
import Modal from '../Modals/AddSecretModal';
import { authObject } from '../../pages/api/models/auth.object';
import { useRouter } from 'next/router';
import { create_secret } from '../../pages/api/secret/secret';
interface FloatingAddButtonProps {
  onAdd: () => void;
}

function FloatingAddButton({onAdd} : FloatingAddButtonProps) {
    const [isModalOpen, setIsModalOpen] = useState(false);

    const handleButtonClick = () => {
        setIsModalOpen(true);
      };
    
      const handleCloseModal = () => {
        setIsModalOpen(false);
      };

  return (
    <div className="floating-add-button">
      <button className="add-button" onClick={handleButtonClick}>+</button>
      {isModalOpen && (
        <Modal onClose={handleCloseModal} onAdd={onAdd} />
      )}
    </div>
  );
};

export default FloatingAddButton;
