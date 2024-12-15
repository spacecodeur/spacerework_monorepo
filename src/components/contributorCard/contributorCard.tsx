import type contributorCardType from "./types/contributorCardType";
import styles from '../../components/contributorCard/styles/global.module.scss';

export default function ContributorCard({picture, pseudo, email, networks}: contributorCardType){

    return (
        <section className={styles.card}>
            <img src={"/contributors/" + picture} alt={"photo de " + pseudo} />
            <h3>{pseudo}</h3>
            <a href={"mailto:" + email}>email</a>
            
            {networks !== undefined &&
                <section>
                    <h4>Mes réseaux</h4>
                    <ul>
                        {
                            Object.entries(networks).map(([networkName, networkUrl], index) =>
                                <li key={index}>
                                    <a href={networkUrl}>{networkName}</a>
                                </li>
                            )
                        }
                    </ul>
                </section>
            }

        </section>
    )

}