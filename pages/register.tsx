import Head from 'next/head'
import Image from 'next/image'
import SimpleReactValidator from 'simple-react-validator';

// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
import { useState } from 'react';
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoker = window.__TAURI__.invoke


export default function Register() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const validator = new SimpleReactValidator();


  function completeForm() {
    if (validator.allValid())
    {
      invoker('command_register_user', {
        username: name,
        password: password,
        email: email
      }).then(() => {
        console.log('test')
      })
    } else {
      validator.showMessages();
      forceUpdate();
    }
    
  }

  const forceUpdate = () => {
    setName(name);
    setEmail(email);
    setPassword(password);
  }

  return (
    <div className="rounded-xl bg-white bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
      <div className="text-white">
        <div className="mb-8 flex flex-col items-center">
          <h1 className="mb-2 text-2xl">1Pass</h1>
          <span className="text-gray-300">Register</span>
        </div>
          <div className="mb-4 text-lg">
            <input type="text" name="Email" placeholder="youremail@email.com" />
            {<a>validator.message('email', email, 'required|email')</a>}
          </div>
          <div className="mb-4 text-lg">
            <input  type="text" name="Username" placeholder="Username" />
            {<a>validator.message('name', name, 'required')</a>}
          </div>
          <div className="mb-4 text-lg">
            <input type="Password" name="Password" placeholder="*********" />
            {<a>validator.message('password', password, 'required|min:8')</a>}
          </div>
          <div className="mt-8 flex justify-center text-lg text-black">
            <button onClick={(e) => completeForm()}>Register</button>
          </div>
        
      </div>
    </div>
  )
}
