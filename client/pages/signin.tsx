import { useState } from 'react'
import { useDispatch } from 'react-redux'
import { SlideFade, Flex, Heading, Input, Button, useColorModeValue } from '@chakra-ui/react'
import { signInRequest } from '../store/user/actions'
import { Formik, Form, Field } from 'formik'
import { FormControl, FormErrorMessage } from '@chakra-ui/react'

const SignIn = () => {
  const formBackground = useColorModeValue('gray.200', 'gray.700')
  const dispatch = useDispatch()

  return (
    <SlideFade in offsetY='20px'>
      <Flex height='100vh' alignItems='center' justifyContent='center'>
        <Flex direction='column' background={formBackground} p={12} rounded={6}>
          <Heading mb={6}>Sign In</Heading>

            <Formik
              initialValues={{
                username: '',
                password: '',
                email: '',
                firstname: '',
                lastname: '',
              }}
              onSubmit={(values, actions) => {
                const { username, password } = values
                dispatch(signInRequest(username, password))
              }}
            >
              <Form>

                <Field name='username'>
                  {({ field, form }) => (
                    <FormControl isInvalid={form.errors.username && form.touched.username}>
                      <Input {...field} id='username' placeholder='Username' type='username' mb={3} variant='filled'/>
                      <FormErrorMessage>{form.errors.username}</FormErrorMessage>
                    </FormControl>
                  )}
                </Field>


                <Field name='password'>
                  {({ field, form }) => (
                    <FormControl isInvalid={form.errors.password && form.touched.password}>
                      <Input {...field} id='password' placeholder='Password' type='password' mb={3} variant='filled'/>
                      <FormErrorMessage>{form.errors.password}</FormErrorMessage>
                    </FormControl>
                  )}
                </Field>
              
                <Button mt={3} colorScheme='blue' type='submit'>Sign In</Button>

              </Form>
            
            </Formik>

        </Flex>
      </Flex>
    </SlideFade>
  )
}

export default SignIn
