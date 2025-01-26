import type { JSX } from 'react'
import LessonView from './components/LessonView';

export default function ReadMode({path, lesson, lesson_html}): JSX.Element {
    
    console.log(lesson_html);
    
    return (
        <>
            <LessonView content={lesson_html}/>
        </>
    )
}