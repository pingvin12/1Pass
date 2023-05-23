import { invoke } from '@tauri-apps/api/tauri'

const isClient = typeof window !== 'undefined'

export const authenticate = async (credentials: any) => {
    let username = credentials.username;
    let password = credentials.password
    
    const res = isClient && invoke('auth_user', {
        username: username,
        password: password,
    })

    return res
}

export const register = async (credentials: any) => {
    let username = credentials.username;
    let password = credentials.password;
    let email = credentials.email;
    
    const res = isClient && invoke('register_user', {
        email: email,
        username: username,
        password: password,
    })

    return res
}