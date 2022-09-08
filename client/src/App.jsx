import React from 'react'
import Cookies from 'universal-cookie'
import { StreamChat } from 'stream-chat'
import { Chat } from 'stream-chat-react'
// import Cookies from 'universal-cookie'

import { ChannelListContainer, ChannelContainer, Auth } from './components'

import './App.css'

const cookies = new Cookies()

const apiKey = 'zw8ykaarrkfm'
const authToken = cookies.get('token')

const client = StreamChat.getInstance(apiKey)

if (authToken) {
    client.connectUser({
          id: cookies.set('userId'),
          name: cookies.set('username'),
          fullName: cookies.set('fullName'),
          image: cookies.set('avatarURL'),
          hashedPassword: cookies.set('hashedPassword'),
          phoneNumber: cookies.set('phoneNumber'),
        }, authToken)
}

const App = () => {
    if (!authToken) return <Auth />
    

  return (
    <div className='app__wrapper'>
        <Chat client={client}>
            <ChannelListContainer />
            <ChannelContainer />
        </Chat>
    </div>
  )
}

export default App