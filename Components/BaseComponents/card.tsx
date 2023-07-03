import React, { useState } from 'react';
import Image from 'next/image';
import closedeye from '../../assets/img/closed-eye.png';
import openedeye from '../../assets/img/opened-eye.png';
import CardRemoveButton from './card_remove_btn';
interface CardProps {
  title: string;
  description: string;
  id: number;
  onRemove(): void;
}

const Card: React.FC<CardProps> = ({ title, description, id, onRemove }) => {
  const [showDescription, setShowDescription] = useState(false);

  const toggleDescription = () => {
    setShowDescription(!showDescription);
    console.log(id);
  };

  return (
    <div className="card">
      <div className="card-content">
        <h2 className="card-title">{title}</h2><CardRemoveButton id={id} onRemove={onRemove} />
        {showDescription ? (
            <>
          <p className="card-description">{description}</p>
          <button className="show-description-button" onClick={toggleDescription}>
            <Image src={openedeye} alt={'secret'} width={25} height={25}/>
          </button>
          </>
        ) : (
            <>
            <p className="card-description">****</p>
          <button className="show-description-button" onClick={toggleDescription}>
            <Image src={closedeye} alt={'secret'} width={25} height={25}/>
          </button>
          </>
        )}
      </div>
    </div>
  );
};

export default Card;