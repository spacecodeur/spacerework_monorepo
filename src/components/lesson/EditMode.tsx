import { useEffect, useState, type JSX } from 'react'
import LessonView from './components/LessonView';
import init, { md_to_html } from "../../wasm/md_to_html.js"

export default function EditMode({path, lesson}): JSX.Element {
    
    const [lesson_html, setLesson_html] = useState<String|null>(null);

    useEffect(()=>{
        init().then(()=>{
            setLesson_html(md_to_html(lesson.content))
        })
    },[]);

    console.log(lesson);
    
    return (
        <>
            <label>
                Éditeur : 
                <textarea 
                    name="postContent" 
                    defaultValue={lesson.content}
                    onChange={(event)=>{
                        setLesson_html(md_to_html(event.target.value));
                    }}
                />
            </label>
            <LessonView content={lesson_html}/>
        </>
    )
}