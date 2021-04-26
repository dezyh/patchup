import Link from 'next/link'
import { Box, Button, Text, Flex, Stack } from '@chakra-ui/react'
import { useSelector } from 'react-redux'
import { useDispatch } from 'react-redux'
import { RootState } from '../store'
import { UserState } from '../store/user/reducer'
import { signOutRequest } from '../store/user/actions'

const Profile = ({ user }) => {
  const dispatch = useDispatch()

  return (
    <Box p={4}>
      <Button onClick={() => dispatch(signOutRequest())}>Sign Out</Button>
    </Box>
  )

}

const Auth = ({ user }) => {

  if (user.token) {
    return (
      <Profile user={user} />
    )
  }

  return (
    <Stack direction='row' padding={4}>
      
      <Link href='/signup'>
        <Button size='md' colorScheme='blue'>Sign Up</Button>
      </Link>

      <Link href='/signin'>
        <Button size='md'>Sign In</Button>
      </Link>

    </Stack>
  )
}

const NavBar = () => {
  const user = useSelector<RootState, UserState>(store => store.user)
  
  return (
    <Flex 
      height='65px'
      bg='transparent'
      flexDirection='row-reverse'
      alignItems='center'
      alignContent='center'
    >
      <Auth user={user} />
    </Flex>             
  )
}

export default NavBar

