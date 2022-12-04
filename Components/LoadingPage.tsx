import styles from '../styles/Home.module.css'
import { BallTriangle } from '@agney/react-loading';
export default function LoadingPage()
{
    return(
        <div className={styles.loadingpage}>
            <h1>rodeo</h1>
            <BallTriangle width={50} color={"#B2FFA9"}/>
            <p>Loading</p>
        </div>
    )
}

