import Button from '@components/button'
import ReloadPrompt from '@components/reload'
import { ThemeContext } from '@context/theme'

import { createApp } from './reactivity'

const ReloadPromptCheck = typeof window !== 'undefined' ?
  () => <ReloadPrompt />
  :
  () => null

export default function() {

  const {
    nextLanguage,
  } = createApp()

  const [theme, { toggleTheme }] = useContext(ThemeContext)

  return (
    <div class="full flex-center flex-col bg-gray-100/75 dark:bg-gray-800">
      <ReloadPromptCheck />
      <div class="flex items-end space-x-6">
        <Button onClick={toggleTheme}>
          {() => {
            return theme.name === 'dark' ?
              <div class="i-carbon-sun w-6 h-6" /> :
              <div class="i-carbon-moon w-6 h-6" />
          }}
        </Button>
        <Button onClick={() => nextLanguage()}>
          <div class="i-carbon-language w-6 h-6" />
        </Button>
      </div>
    </div>
  )
}
