import React from 'react';
import { delete_secret } from '../../pages/api/secret/secret';

interface RemoveButtonProps {
    id: number;
  onRemove: () => void;
}

function CardRemoveButton({onRemove, id} : RemoveButtonProps) {

    const handleButtonClick = async () => {
        await delete_secret(id);
        onRemove();
      };

  return (
    <div className="card-remove-button">
      <button className="remove-button" onClick={handleButtonClick}>-</button>
    </div>
  );
};

export default CardRemoveButton;
