import { type JSX } from 'react'
import style from "../../styles/components/local/Navbar.module.scss"

export default function Navbar(): JSX.Element {
    return (
        <nav className={style.navbar_container}>
         <p>SpaceRework</p>
        </nav>
    )
}