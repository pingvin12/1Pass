import Head from 'next/head'
import Image from 'next/image'

// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoker = window.__TAURI__.invoke


export default function Register() {
  function completeForm() {

    invoker('command_register_user', {
      username: 'user',
      password: 'password',
      email: 'email'
    }).then(() => {
      console.log('test')
    })
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
          </div>
          <div className="mb-4 text-lg">
            <input  type="text" name="Username" placeholder="Username" />
          </div>
          <div className="mb-4 text-lg">
            <input type="Password" name="Password" placeholder="*********" />
          </div>
          <div className="mt-8 flex justify-center text-lg text-black">
            <button onClick={completeForm}>Register</button>
          </div>
        
      </div>
    </div>
  )
}
